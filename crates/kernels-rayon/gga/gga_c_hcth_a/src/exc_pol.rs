//! GGA_C_HCTH_A exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_hcth_a.c`
//! by tools/translate_rayon/from_maple.py, then rewritten to
//! `wide::f64x8` by simd.py. Eight grid points per step; every lane runs maple2c's expression
//! sequence in its original order.
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]
use libxc_rkernel_math::constants::*;
use libxc_rkernel_math::simd;
use libxc_rkernel_math::wide::{f64x8, CmpEq, CmpGe, CmpGt, CmpLe, CmpLt, CmpNe};

const V_ZERO: f64x8 = f64x8::new([0.0; 8]);
const V_ONE: f64x8 = f64x8::new([1.0; 8]);

// Transcendentals in exact mode come from `libxc_rkernel_math::simd`,
// which is bit-identical / correctly-rounded per lane to the scalar calls
// the scalar kernel makes. In exact mode, the SIMD kernel produces output
// bit-identical to its scalar form.

/// Load 8 consecutive grid points.
///
/// The tail is padded by repeating the last element, not by zero-filling:
/// these formulas divide by rho, so a zero lane would raise inf/NaN in lanes
/// whose results are then discarded -- harmless to the answer, but it makes
/// any real NaN impossible to spot while debugging.
#[inline(always)]
fn load(s: &[f64], ip: usize, np: usize) -> f64x8 {
    if ip + 8 <= np {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        f64x8::new(b)
    } else {
        let mut b = [s[np - 1]; 8];
        b[..np - ip].copy_from_slice(&s[ip..np]);
        f64x8::new(b)
    }
}

/// Accumulate 8 consecutive grid points into an output array.
///
/// `+=`, not `=`. The scalar kernel writes `out[ip] += v`; a plain store is a
/// different operation in two ways. It keeps the sign of a negative zero where
/// `0.0 + -0.0` gives `+0.0` -- a bit difference the fingerprint gate reports
/// as a rejection even though no value changed (`gga_x_pbepow fxc` was
/// rejected on exactly this, 273 of 200,000 `v2sigma2` elements) -- and it
/// would discard whatever a caller had already put in the buffer.
#[inline(always)]
fn store_add(s: &mut [f64], ip: usize, m: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        let r: [f64; 8] = (f64x8::new(b) + acc).into();
        s[ip..ip + 8].copy_from_slice(&r);
    } else {
        for k in 0..m {
            s[ip + k] += a[k];
        }
    }
}

/// Load 8 elements with a given stride and offset.
#[inline(always)]
fn load_strided(s: &[f64], ip: usize, np: usize, stride: usize, offset: usize) -> f64x8 {
    let mut b = [0.0f64; 8];
    if ip + 8 <= np {
        let base = ip * stride + offset;
        b[0] = s[base];
        b[1] = s[base + stride];
        b[2] = s[base + 2 * stride];
        b[3] = s[base + 3 * stride];
        b[4] = s[base + 4 * stride];
        b[5] = s[base + 5 * stride];
        b[6] = s[base + 6 * stride];
        b[7] = s[base + 7 * stride];
    } else {
        for k in 0..8 {
            let p = (ip + k).min(np - 1);
            b[k] = s[p * stride + offset];
        }
    }
    f64x8::new(b)
}

/// Accumulate 8 elements with a given stride and offset.
///
/// `+=`, not `=`: the scalar kernel this was translated from writes
/// `out[ip * stride + offset] += v`, and a plain store is not the same
/// operation. It differs on the sign of zero -- `0.0 + -0.0` is `+0.0`
/// while a store of `-0.0` keeps the sign -- which is a bit difference
/// the fingerprint gate sees, and it would silently drop a caller's
/// existing contribution if one were ever there.
///
/// The read is not free on this path: a polarized `kxc`/`lxc` kernel
/// writes many strided outputs per point, and `lda_c_pw_erf kxc pol`
/// measured 84 -> 114 ns/pt (1.36x). It is charged anyway, because the
/// scalar kernel this is compared against does the same read. Gathering
/// into a vector, adding once and scattering back was tried and is no
/// faster (117 ns/pt), so the cost is the load itself, not scheduling.
#[inline(always)]
fn store_strided(s: &mut [f64], ip: usize, m: usize, stride: usize, offset: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let base = ip * stride + offset;
        s[base] += a[0];
        s[base + stride] += a[1];
        s[base + 2 * stride] += a[2];
        s[base + 3 * stride] += a[3];
        s[base + 4 * stride] += a[4];
        s[base + 5 * stride] += a[5];
        s[base + 6 * stride] += a[6];
        s[base + 7 * stride] += a[7];
    } else {
        for k in 0..m {
            s[(ip + k) * stride + offset] += a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_hcth_a_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho0 = load_strided(rho, ip, np, 2, 0);
        let v_rho1 = load_strided(rho, ip, np, 2, 1);
        let v_sigma0 = load_strided(sigma, ip, np, 3, 0);
        let v_sigma1 = load_strided(sigma, ip, np, 3, 1);
        let v_sigma2 = load_strided(sigma, ip, np, 3, 2);
        let mut acc_zk = V_ZERO;
        {
            let t2 = v_rho0 - v_rho1;
            let t3 = v_rho0 + v_rho1;
            let t4 = f64x8::splat(1.0) / t3;
            let t5 = t2 * t4;
            let t6 = f64x8::splat(1.0) + t5;
            let t7 = (t6).simd_le(zeta_threshold);
            let t8 = ((v_rho0).simd_le(dens_threshold)) | (t7);
            let t9 = ((t7).select(zeta_threshold, t6));
            let t10 = f64x8::splat(M_CBRT3);
            let t11 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t12 = (simd::cbrt(t11));
            let t13 = t10 * t12;
            let t14 = f64x8::splat(M_CBRT4);
            let t15 = t14 * t14;
            let t16 = t13 * t15;
            let t17 = (simd::cbrt(t3));
            let t18 = f64x8::splat(1.0) / t17;
            let t19 = f64x8::splat(M_CBRT2);
            let t20 = t18 * t19;
            let t21 = (simd::cbrt(zeta_threshold));
            let t22 = f64x8::splat(1.0) / t21;
            let t23 = (simd::cbrt(t6));
            let t25 = ((t7).select(t22, f64x8::splat(1.0) / t23));
            let t27 = t16 * t20 * t25;
            let t28 = t27 / f64x8::splat(4.0);
            let t29 = ((t27).sqrt());
            let t31 = t28 + f64x8::splat(1.86372) * t29 + f64x8::splat(12.9352);
            let t32 = f64x8::splat(1.0) / t31;
            let t33 = t25 * t32;
            let t37 = (simd::ln(t16 * t20 * t33 / f64x8::splat(4.0)));
            let t38 = f64x8::splat(0.0310907) * t37;
            let t39 = t29 + f64x8::splat(3.72744);
            let t42 = (simd::atan(f64x8::splat(6.15199081975908) / t39));
            let t43 = f64x8::splat(0.038783294878113016) * t42;
            let t44 = t29 / f64x8::splat(2.0);
            let t45 = t44 + f64x8::splat(0.10498);
            let t46 = t45 * t45;
            let t48 = (simd::ln(t46 * t32));
            let t49 = f64x8::splat(0.0009690227711544374) * t48;
            let t51 = t28 + f64x8::splat(3.53021) * t29 + f64x8::splat(18.0578);
            let t52 = f64x8::splat(1.0) / t51;
            let t53 = t25 * t52;
            let t57 = (simd::ln(t16 * t20 * t53 / f64x8::splat(4.0)));
            let t59 = t29 + f64x8::splat(7.06042);
            let t62 = (simd::atan(f64x8::splat(4.730926909560113) / t59));
            let t64 = t44 + f64x8::splat(0.325);
            let t65 = t64 * t64;
            let t67 = (simd::ln(t65 * t52));
            let t71 = t21 * zeta_threshold;
            let t73 = (((f64x8::splat(2.0)).simd_le(zeta_threshold)).select(t71, f64x8::splat(2.0) * t19));
            let t75 = (((f64x8::splat(0.0)).simd_le(zeta_threshold)).select(t71, f64x8::splat(0.0)));
            let t76 = t73 + t75 - f64x8::splat(2.0);
            let t78 = t19 - f64x8::splat(1.0);
            let t80 = f64x8::splat(1.0) / t78 / f64x8::splat(2.0);
            let t82 = t38 + t43 + t49 + (f64x8::splat(0.01554535) * t57 + f64x8::splat(0.05249139316978094) * t62 + f64x8::splat(0.0022478670955426118) * t67 - t38 - t43 - t49) * t76 * t80;
            let t85 = ((t8).select(f64x8::splat(0.0), t9 * t82 / f64x8::splat(2.0)));
            let t86 = v_rho0 * v_rho0;
            let t87 = (simd::cbrt(v_rho0));
            let t88 = t87 * t87;
            let t90 = f64x8::splat(1.0) / t88 / t86;
            let t91 = v_sigma0 * t90;
            let t93 = f64x8::splat(1.0) + f64x8::splat(0.2) * t91;
            let t94 = f64x8::splat(1.0) / t93;
            let t97 = v_sigma0 * v_sigma0;
            let t98 = t86 * t86;
            let t99 = t98 * v_rho0;
            let t101 = f64x8::splat(1.0) / t87 / t99;
            let t103 = t93 * t93;
            let t104 = f64x8::splat(1.0) / t103;
            let t107 = t97 * v_sigma0;
            let t108 = t98 * t98;
            let t109 = f64x8::splat(1.0) / t108;
            let t111 = t103 * t93;
            let t112 = f64x8::splat(1.0) / t111;
            let t115 = f64x8::splat(0.0136823) + f64x8::splat(0.053784) * t91 * t94 - f64x8::splat(0.02203076) * t97 * t101 * t104 + f64x8::splat(0.00831576) * t107 * t109 * t112;
            let t116 = t85 * t115;
            let t118 = f64x8::splat(1.0) - t5;
            let t119 = (t118).simd_le(zeta_threshold);
            let t120 = ((v_rho1).simd_le(dens_threshold)) | (t119);
            let t121 = ((t119).select(zeta_threshold, t118));
            let t122 = (simd::cbrt(t118));
            let t124 = ((t119).select(t22, f64x8::splat(1.0) / t122));
            let t126 = t16 * t20 * t124;
            let t127 = t126 / f64x8::splat(4.0);
            let t128 = ((t126).sqrt());
            let t130 = t127 + f64x8::splat(1.86372) * t128 + f64x8::splat(12.9352);
            let t131 = f64x8::splat(1.0) / t130;
            let t132 = t124 * t131;
            let t136 = (simd::ln(t16 * t20 * t132 / f64x8::splat(4.0)));
            let t137 = f64x8::splat(0.0310907) * t136;
            let t138 = t128 + f64x8::splat(3.72744);
            let t141 = (simd::atan(f64x8::splat(6.15199081975908) / t138));
            let t142 = f64x8::splat(0.038783294878113016) * t141;
            let t143 = t128 / f64x8::splat(2.0);
            let t144 = t143 + f64x8::splat(0.10498);
            let t145 = t144 * t144;
            let t147 = (simd::ln(t145 * t131));
            let t148 = f64x8::splat(0.0009690227711544374) * t147;
            let t150 = t127 + f64x8::splat(3.53021) * t128 + f64x8::splat(18.0578);
            let t151 = f64x8::splat(1.0) / t150;
            let t152 = t124 * t151;
            let t156 = (simd::ln(t16 * t20 * t152 / f64x8::splat(4.0)));
            let t158 = t128 + f64x8::splat(7.06042);
            let t161 = (simd::atan(f64x8::splat(4.730926909560113) / t158));
            let t163 = t143 + f64x8::splat(0.325);
            let t164 = t163 * t163;
            let t166 = (simd::ln(t164 * t151));
            let t171 = t137 + t142 + t148 + (f64x8::splat(0.01554535) * t156 + f64x8::splat(0.05249139316978094) * t161 + f64x8::splat(0.0022478670955426118) * t166 - t137 - t142 - t148) * t76 * t80;
            let t174 = ((t120).select(f64x8::splat(0.0), t121 * t171 / f64x8::splat(2.0)));
            let t175 = v_rho1 * v_rho1;
            let t176 = (simd::cbrt(v_rho1));
            let t177 = t176 * t176;
            let t179 = f64x8::splat(1.0) / t177 / t175;
            let t180 = v_sigma2 * t179;
            let t182 = f64x8::splat(1.0) + f64x8::splat(0.2) * t180;
            let t183 = f64x8::splat(1.0) / t182;
            let t186 = v_sigma2 * v_sigma2;
            let t187 = t175 * t175;
            let t188 = t187 * v_rho1;
            let t190 = f64x8::splat(1.0) / t176 / t188;
            let t192 = t182 * t182;
            let t193 = f64x8::splat(1.0) / t192;
            let t196 = t186 * v_sigma2;
            let t197 = t187 * t187;
            let t198 = f64x8::splat(1.0) / t197;
            let t200 = t192 * t182;
            let t201 = f64x8::splat(1.0) / t200;
            let t204 = f64x8::splat(0.0136823) + f64x8::splat(0.053784) * t180 * t183 - f64x8::splat(0.02203076) * t186 * t190 * t193 + f64x8::splat(0.00831576) * t196 * t198 * t201;
            let t205 = t174 * t204;
            let t206 = t15 * t18;
            let t207 = t13 * t206;
            let t208 = t207 / f64x8::splat(4.0);
            let t209 = ((t207).sqrt());
            let t211 = t208 + f64x8::splat(1.86372) * t209 + f64x8::splat(12.9352);
            let t212 = f64x8::splat(1.0) / t211;
            let t216 = (simd::ln(t13 * t206 * t212 / f64x8::splat(4.0)));
            let t217 = f64x8::splat(0.0310907) * t216;
            let t218 = t209 + f64x8::splat(3.72744);
            let t221 = (simd::atan(f64x8::splat(6.15199081975908) / t218));
            let t222 = f64x8::splat(0.038783294878113016) * t221;
            let t223 = t209 / f64x8::splat(2.0);
            let t224 = t223 + f64x8::splat(0.10498);
            let t225 = t224 * t224;
            let t227 = (simd::ln(t225 * t212));
            let t228 = f64x8::splat(0.0009690227711544374) * t227;
            let t229 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t230 = f64x8::splat(1.0) / t229;
            let t232 = t208 + f64x8::splat(0.565535) * t209 + f64x8::splat(13.0045);
            let t233 = f64x8::splat(1.0) / t232;
            let t237 = (simd::ln(t13 * t206 * t233 / f64x8::splat(4.0)));
            let t238 = t209 + f64x8::splat(1.13107);
            let t241 = (simd::atan(f64x8::splat(7.123108917818118) / t238));
            let t243 = t223 + f64x8::splat(0.0047584);
            let t244 = t243 * t243;
            let t246 = (simd::ln(t244 * t233));
            let t249 = t230 * (t237 + f64x8::splat(0.31770800474394145) * t241 + f64x8::splat(0.00041403379428206277) * t246);
            let t250 = t23 * t6;
            let t251 = ((t7).select(t71, t250));
            let t252 = t122 * t118;
            let t253 = ((t119).select(t71, t252));
            let t254 = t251 + t253 - f64x8::splat(2.0);
            let t255 = t249 * t254;
            let t256 = t2 * t2;
            let t257 = t256 * t256;
            let t258 = t3 * t3;
            let t259 = t258 * t258;
            let t260 = f64x8::splat(1.0) / t259;
            let t264 = f64x8::splat(9.0) * t78;
            let t265 = t80 * (-t257 * t260 + f64x8::splat(1.0)) * t264;
            let t269 = t208 + f64x8::splat(3.53021) * t209 + f64x8::splat(18.0578);
            let t270 = f64x8::splat(1.0) / t269;
            let t274 = (simd::ln(t13 * t206 * t270 / f64x8::splat(4.0)));
            let t276 = t209 + f64x8::splat(7.06042);
            let t279 = (simd::atan(f64x8::splat(4.730926909560113) / t276));
            let t281 = t223 + f64x8::splat(0.325);
            let t282 = t281 * t281;
            let t284 = (simd::ln(t282 * t270));
            let t286 = f64x8::splat(0.01554535) * t274 + f64x8::splat(0.05249139316978094) * t279 + f64x8::splat(0.0022478670955426118) * t284 - t217 - t222 - t228;
            let t287 = t286 * t254;
            let t288 = t80 * t257;
            let t289 = t288 * t260;
            let t291 = t217 + t222 + t228 - t255 * t265 / f64x8::splat(24.0) + t287 * t289 - t85 - t174;
            let t292 = t91 + t180;
            let t295 = f64x8::splat(1.0) + f64x8::splat(0.003) * t91 + f64x8::splat(0.003) * t180;
            let t296 = f64x8::splat(1.0) / t295;
            let t299 = t292 * t292;
            let t300 = t295 * t295;
            let t301 = f64x8::splat(1.0) / t300;
            let t304 = t299 * t292;
            let t305 = t300 * t295;
            let t306 = f64x8::splat(1.0) / t305;
            let t309 = f64x8::splat(0.836897) + f64x8::splat(0.00516153) * t292 * t296 - f64x8::splat(2.506482e-05) * t299 * t301 - f64x8::splat(1.2352608e-07) * t304 * t306;
            let t310 = t291 * t309;
            let tzk0 = t116 + t205 + t310;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}
