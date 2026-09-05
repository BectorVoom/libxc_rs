//! GGA_X_SFAT vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_sfat.c`
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
pub fn gga_x_sfat_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_hyb_omega_0 = f64x8::splat(param_hyb_omega_0);
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
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        let mut acc_vsigma_0 = V_ZERO;
        let mut acc_vsigma_1 = V_ZERO;
        let mut acc_vsigma_2 = V_ZERO;
        {
            let t1 = (v_rho0).simd_le(dens_threshold);
            let t2 = f64x8::splat(M_CBRT3);
            let t3 = f64x8::splat(M_CBRTPI);
            let t5 = f64x8::splat(1.0) / t3 * t2;
            let t6 = v_rho0 + v_rho1;
            let t7 = f64x8::splat(1.0) / t6;
            let t10 = (f64x8::splat(2.0) * t7 * v_rho0).simd_le(zeta_threshold);
            let t11 = zeta_threshold - f64x8::splat(1.0);
            let t14 = (f64x8::splat(2.0) * t7 * v_rho1).simd_le(zeta_threshold);
            let t15 = -t11;
            let t16 = v_rho0 - v_rho1;
            let t18 = ((t10).select(t11, (t14).select(t15, t7 * t16)));
            let t19 = f64x8::splat(1.0) + t18;
            let t20 = (t19).simd_le(zeta_threshold);
            let t21 = (simd::cbrt(zeta_threshold));
            let t22 = t21 * zeta_threshold;
            let t23 = (simd::cbrt(t19));
            let t25 = ((t20).select(t22, t23 * t19));
            let t26 = t25 * t5;
            let t27 = (simd::cbrt(t6));
            let t28 = t2 * t2;
            let t29 = t28 * f64x8::splat(M_PI);
            let t30 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t31 = (simd::cbrt(t30));
            let t32 = f64x8::splat(1.0) / t31;
            let t33 = f64x8::splat(M_CBRT4);
            let t34 = t33 * t32;
            let t35 = t32 * t28;
            let t36 = t33 * t35;
            let t37 = v_rho0 * v_rho0;
            let t38 = (simd::cbrt(v_rho0));
            let t39 = t38 * t38;
            let t41 = f64x8::splat(1.0) / t39 / t37;
            let t42 = t41 * v_sigma0;
            let t43 = ((v_sigma0).sqrt());
            let t45 = f64x8::splat(1.0) / t38 / v_rho0;
            let t46 = t45 * t43;
            let t47 = (simd::ln(t46 + ((t46 * t46 + f64x8::splat(1.0)).sqrt())));
            let t50 = f64x8::splat(1.0) + f64x8::splat(0.0252) * t47 * t46;
            let t51 = f64x8::splat(1.0) / t50;
            let t55 = f64x8::splat(1.0) + f64x8::splat(0.0009333333333333333) * t51 * t42 * t36;
            let t58 = f64x8::splat(1.0) / t55 * t34 * t29;
            let t59 = ((t58).sqrt());
            let t61 = f64x8::splat(1.0) / t59 * param_hyb_omega_0;
            let t62 = f64x8::splat(M_CBRT2);
            let t63 = t6 * t19;
            let t64 = (simd::cbrt(t63));
            let t65 = f64x8::splat(1.0) / t64;
            let t66 = t65 * t62;
            let t68 = t66 * t61 / f64x8::splat(2.0);
            let t69 = (f64x8::splat(1.92)).simd_le(t68);
            let t70 = (f64x8::splat(1.92)).simd_lt(t68);
            let t71 = ((t70).select(t68, f64x8::splat(1.92)));
            let t72 = t71 * t71;
            let t73 = t72 * t72;
            let t74 = f64x8::splat(1.0) / t73;
            let t76 = t73 * t72;
            let t77 = f64x8::splat(1.0) / t76;
            let t79 = t73 * t73;
            let t80 = f64x8::splat(1.0) / t79;
            let t82 = t79 * t72;
            let t83 = f64x8::splat(1.0) / t82;
            let t85 = t79 * t73;
            let t86 = f64x8::splat(1.0) / t85;
            let t88 = t79 * t76;
            let t89 = f64x8::splat(1.0) / t88;
            let t91 = t79 * t79;
            let t92 = f64x8::splat(1.0) / t91;
            let t95 = f64x8::splat(1.0) / t91 / t72;
            let t98 = f64x8::splat(1.0) / t91 / t73;
            let t101 = f64x8::splat(1.0) / t91 / t76;
            let t104 = f64x8::splat(1.0) / t91 / t79;
            let t107 = f64x8::splat(1.0) / t91 / t82;
            let t110 = f64x8::splat(1.0) / t91 / t85;
            let t113 = f64x8::splat(1.0) / t91 / t88;
            let t115 = t91 * t91;
            let t116 = f64x8::splat(1.0) / t115;
            let t119 = f64x8::splat(1.0) / t115 / t72;
            let t122 = f64x8::splat(1.0) / t115 / t73;
            let t126 = -t74 / f64x8::splat(30.0) + t77 / f64x8::splat(70.0) - t80 / f64x8::splat(135.0) + t83 / f64x8::splat(231.0) - t86 / f64x8::splat(364.0) + t89 / f64x8::splat(540.0) - t92 / f64x8::splat(765.0) + t95 / f64x8::splat(1045.0) - t98 / f64x8::splat(1386.0) + t101 / f64x8::splat(1794.0) - t104 / f64x8::splat(2275.0) + t107 / f64x8::splat(2835.0) - t110 / f64x8::splat(3480.0) + t113 / f64x8::splat(4216.0) - t116 / f64x8::splat(5049.0) + t119 / f64x8::splat(5985.0) - t122 / f64x8::splat(7030.0) + f64x8::splat(1.0) / t72 / f64x8::splat(9.0);
            let t127 = ((t70).select(f64x8::splat(1.92), t68));
            let t128 = (simd::atan2(f64x8::splat(1.0), t127));
            let t129 = t127 * t127;
            let t130 = t129 + f64x8::splat(3.0);
            let t131 = f64x8::splat(1.0) / t129;
            let t132 = f64x8::splat(1.0) + t131;
            let t133 = (simd::ln(t132));
            let t135 = -t133 * t130 + f64x8::splat(1.0);
            let t138 = t128 + t135 * t127 / f64x8::splat(4.0);
            let t142 = ((t69).select(t126, f64x8::splat(1.0) - f64x8::splat(8.0) / f64x8::splat(3.0) * t138 * t127));
            let t143 = t142 * t27;
            let t144 = t55 * t143;
            let t147 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t144 * t26));
            let t148 = (v_rho1).simd_le(dens_threshold);
            let t149 = -t16;
            let t151 = ((t14).select(t11, (t10).select(t15, t7 * t149)));
            let t152 = f64x8::splat(1.0) + t151;
            let t153 = (t152).simd_le(zeta_threshold);
            let t154 = (simd::cbrt(t152));
            let t156 = ((t153).select(t22, t154 * t152));
            let t157 = t156 * t5;
            let t158 = v_rho1 * v_rho1;
            let t159 = (simd::cbrt(v_rho1));
            let t160 = t159 * t159;
            let t162 = f64x8::splat(1.0) / t160 / t158;
            let t163 = t162 * v_sigma2;
            let t164 = ((v_sigma2).sqrt());
            let t166 = f64x8::splat(1.0) / t159 / v_rho1;
            let t167 = t166 * t164;
            let t168 = (simd::ln(t167 + ((t167 * t167 + f64x8::splat(1.0)).sqrt())));
            let t171 = f64x8::splat(1.0) + f64x8::splat(0.0252) * t168 * t167;
            let t172 = f64x8::splat(1.0) / t171;
            let t176 = f64x8::splat(1.0) + f64x8::splat(0.0009333333333333333) * t172 * t163 * t36;
            let t179 = f64x8::splat(1.0) / t176 * t34 * t29;
            let t180 = ((t179).sqrt());
            let t182 = f64x8::splat(1.0) / t180 * param_hyb_omega_0;
            let t183 = t6 * t152;
            let t184 = (simd::cbrt(t183));
            let t185 = f64x8::splat(1.0) / t184;
            let t186 = t185 * t62;
            let t188 = t186 * t182 / f64x8::splat(2.0);
            let t189 = (f64x8::splat(1.92)).simd_le(t188);
            let t190 = (f64x8::splat(1.92)).simd_lt(t188);
            let t191 = ((t190).select(t188, f64x8::splat(1.92)));
            let t192 = t191 * t191;
            let t193 = t192 * t192;
            let t194 = f64x8::splat(1.0) / t193;
            let t196 = t193 * t192;
            let t197 = f64x8::splat(1.0) / t196;
            let t199 = t193 * t193;
            let t200 = f64x8::splat(1.0) / t199;
            let t202 = t199 * t192;
            let t203 = f64x8::splat(1.0) / t202;
            let t205 = t199 * t193;
            let t206 = f64x8::splat(1.0) / t205;
            let t208 = t199 * t196;
            let t209 = f64x8::splat(1.0) / t208;
            let t211 = t199 * t199;
            let t212 = f64x8::splat(1.0) / t211;
            let t215 = f64x8::splat(1.0) / t211 / t192;
            let t218 = f64x8::splat(1.0) / t211 / t193;
            let t221 = f64x8::splat(1.0) / t211 / t196;
            let t224 = f64x8::splat(1.0) / t211 / t199;
            let t227 = f64x8::splat(1.0) / t211 / t202;
            let t230 = f64x8::splat(1.0) / t211 / t205;
            let t233 = f64x8::splat(1.0) / t211 / t208;
            let t235 = t211 * t211;
            let t236 = f64x8::splat(1.0) / t235;
            let t239 = f64x8::splat(1.0) / t235 / t192;
            let t242 = f64x8::splat(1.0) / t235 / t193;
            let t246 = -t194 / f64x8::splat(30.0) + t197 / f64x8::splat(70.0) - t200 / f64x8::splat(135.0) + t203 / f64x8::splat(231.0) - t206 / f64x8::splat(364.0) + t209 / f64x8::splat(540.0) - t212 / f64x8::splat(765.0) + t215 / f64x8::splat(1045.0) - t218 / f64x8::splat(1386.0) + t221 / f64x8::splat(1794.0) - t224 / f64x8::splat(2275.0) + t227 / f64x8::splat(2835.0) - t230 / f64x8::splat(3480.0) + t233 / f64x8::splat(4216.0) - t236 / f64x8::splat(5049.0) + t239 / f64x8::splat(5985.0) - t242 / f64x8::splat(7030.0) + f64x8::splat(1.0) / t192 / f64x8::splat(9.0);
            let t247 = ((t190).select(f64x8::splat(1.92), t188));
            let t248 = (simd::atan2(f64x8::splat(1.0), t247));
            let t249 = t247 * t247;
            let t250 = t249 + f64x8::splat(3.0);
            let t251 = f64x8::splat(1.0) / t249;
            let t252 = f64x8::splat(1.0) + t251;
            let t253 = (simd::ln(t252));
            let t255 = -t253 * t250 + f64x8::splat(1.0);
            let t258 = t248 + t255 * t247 / f64x8::splat(4.0);
            let t262 = ((t189).select(t246, f64x8::splat(1.0) - f64x8::splat(8.0) / f64x8::splat(3.0) * t258 * t247));
            let t263 = t262 * t27;
            let t264 = t176 * t263;
            let t267 = ((t148).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t264 * t157));
            let tzk0 = t147 + t267;
            acc_zk = tzk0;
            let t268 = t6 * t6;
            let t269 = f64x8::splat(1.0) / t268;
            let t270 = t269 * t16;
            let t272 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), t7 - t270)));
            let t275 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t272 * t23));
            let t276 = t275 * t5;
            let t279 = t27 * t27;
            let t280 = f64x8::splat(1.0) / t279;
            let t281 = t142 * t280;
            let t282 = t55 * t281;
            let t284 = t282 * t26 / f64x8::splat(8.0);
            let t285 = t73 * t71;
            let t286 = f64x8::splat(1.0) / t285;
            let t289 = f64x8::splat(1.0) / t59 / t58 * param_hyb_omega_0;
            let t291 = f64x8::splat(M_PI) * t66 * t289;
            let t292 = t55 * t55;
            let t293 = f64x8::splat(1.0) / t292;
            let t294 = t293 * t33;
            let t295 = t37 * v_rho0;
            let t297 = f64x8::splat(1.0) / t39 / t295;
            let t298 = t297 * v_sigma0;
            let t302 = t50 * t50;
            let t303 = f64x8::splat(1.0) / t302;
            let t305 = f64x8::splat(1.0) / t38 / t37;
            let t309 = t42 + f64x8::splat(1.0);
            let t310 = ((t309).sqrt());
            let t311 = f64x8::splat(1.0) / t310;
            let t314 = -f64x8::splat(0.0336) * t47 * t305 * t43 - f64x8::splat(0.0336) * t311 * t298;
            let t315 = t314 * t303;
            let t319 = -f64x8::splat(0.002488888888888889) * t51 * t298 * t36 - f64x8::splat(0.0009333333333333333) * t315 * t42 * t36;
            let t320 = t319 * t294;
            let t325 = f64x8::splat(1.0) / t64 / t63;
            let t326 = t325 * t62;
            let t328 = t6 * t272 + t18 + f64x8::splat(1.0);
            let t332 = t320 * t35 * t291 / f64x8::splat(4.0) - t328 * t326 * t61 / f64x8::splat(6.0);
            let t333 = ((t70).select(t332, f64x8::splat(0.0)));
            let t336 = t72 * t71;
            let t337 = t73 * t336;
            let t338 = f64x8::splat(1.0) / t337;
            let t341 = t79 * t71;
            let t342 = f64x8::splat(1.0) / t341;
            let t345 = t79 * t336;
            let t346 = f64x8::splat(1.0) / t345;
            let t349 = t79 * t285;
            let t350 = f64x8::splat(1.0) / t349;
            let t353 = t79 * t337;
            let t354 = f64x8::splat(1.0) / t353;
            let t358 = f64x8::splat(1.0) / t91 / t71;
            let t362 = f64x8::splat(1.0) / t91 / t336;
            let t366 = f64x8::splat(1.0) / t91 / t285;
            let t370 = f64x8::splat(1.0) / t91 / t337;
            let t374 = f64x8::splat(1.0) / t91 / t341;
            let t378 = f64x8::splat(1.0) / t91 / t345;
            let t382 = f64x8::splat(1.0) / t91 / t349;
            let t386 = f64x8::splat(1.0) / t91 / t353;
            let t390 = f64x8::splat(1.0) / t115 / t71;
            let t394 = f64x8::splat(1.0) / t115 / t336;
            let t398 = f64x8::splat(1.0) / t115 / t285;
            let t401 = f64x8::splat(1.0) / t336;
            let t404 = f64x8::splat(2.0) / f64x8::splat(15.0) * t333 * t286 - f64x8::splat(3.0) / f64x8::splat(35.0) * t333 * t338 + f64x8::splat(8.0) / f64x8::splat(135.0) * t333 * t342 - f64x8::splat(10.0) / f64x8::splat(231.0) * t333 * t346 + f64x8::splat(3.0) / f64x8::splat(91.0) * t333 * t350 - f64x8::splat(7.0) / f64x8::splat(270.0) * t333 * t354 + f64x8::splat(16.0) / f64x8::splat(765.0) * t333 * t358 - f64x8::splat(18.0) / f64x8::splat(1045.0) * t333 * t362 + f64x8::splat(10.0) / f64x8::splat(693.0) * t333 * t366 - f64x8::splat(11.0) / f64x8::splat(897.0) * t333 * t370 + f64x8::splat(24.0) / f64x8::splat(2275.0) * t333 * t374 - f64x8::splat(26.0) / f64x8::splat(2835.0) * t333 * t378 + f64x8::splat(7.0) / f64x8::splat(870.0) * t333 * t382 - f64x8::splat(15.0) / f64x8::splat(2108.0) * t333 * t386 + f64x8::splat(32.0) / f64x8::splat(5049.0) * t333 * t390 - f64x8::splat(34.0) / f64x8::splat(5985.0) * t333 * t394 + f64x8::splat(18.0) / f64x8::splat(3515.0) * t333 * t398 - f64x8::splat(2.0) / f64x8::splat(9.0) * t333 * t401;
            let t405 = ((t70).select(f64x8::splat(0.0), t332));
            let t408 = f64x8::splat(1.0) / t132;
            let t414 = t129 * t127;
            let t415 = f64x8::splat(1.0) / t414;
            let t416 = t415 * t130;
            let t417 = t408 * t405;
            let t420 = -f64x8::splat(2.0) * t133 * t405 * t127 + f64x8::splat(2.0) * t417 * t416;
            let t423 = -t408 * t131 * t405 + t135 * t405 / f64x8::splat(4.0) + t420 * t127 / f64x8::splat(4.0);
            let t427 = ((t69).select(t404, -f64x8::splat(8.0) / f64x8::splat(3.0) * t423 * t127 - f64x8::splat(8.0) / f64x8::splat(3.0) * t138 * t405));
            let t428 = t427 * t27;
            let t429 = t55 * t428;
            let t432 = t319 * t143;
            let t436 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t144 * t276 - t284 - f64x8::splat(3.0) / f64x8::splat(8.0) * t429 * t26 - f64x8::splat(3.0) / f64x8::splat(8.0) * t432 * t26));
            let t437 = t269 * t149;
            let t439 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -t7 - t437)));
            let t442 = ((t153).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t439 * t154));
            let t443 = t442 * t5;
            let t446 = t262 * t280;
            let t447 = t176 * t446;
            let t449 = t447 * t157 / f64x8::splat(8.0);
            let t450 = t193 * t191;
            let t451 = f64x8::splat(1.0) / t450;
            let t453 = f64x8::splat(1.0) / t184 / t183;
            let t454 = t453 * t62;
            let t456 = t6 * t439 + t151 + f64x8::splat(1.0);
            let t459 = t456 * t454 * t182 / f64x8::splat(6.0);
            let t460 = ((t190).select(-t459, f64x8::splat(0.0)));
            let t463 = t192 * t191;
            let t464 = t193 * t463;
            let t465 = f64x8::splat(1.0) / t464;
            let t468 = t199 * t191;
            let t469 = f64x8::splat(1.0) / t468;
            let t472 = t199 * t463;
            let t473 = f64x8::splat(1.0) / t472;
            let t476 = t199 * t450;
            let t477 = f64x8::splat(1.0) / t476;
            let t480 = t199 * t464;
            let t481 = f64x8::splat(1.0) / t480;
            let t485 = f64x8::splat(1.0) / t211 / t191;
            let t489 = f64x8::splat(1.0) / t211 / t463;
            let t493 = f64x8::splat(1.0) / t211 / t450;
            let t497 = f64x8::splat(1.0) / t211 / t464;
            let t501 = f64x8::splat(1.0) / t211 / t468;
            let t505 = f64x8::splat(1.0) / t211 / t472;
            let t509 = f64x8::splat(1.0) / t211 / t476;
            let t513 = f64x8::splat(1.0) / t211 / t480;
            let t517 = f64x8::splat(1.0) / t235 / t191;
            let t521 = f64x8::splat(1.0) / t235 / t463;
            let t525 = f64x8::splat(1.0) / t235 / t450;
            let t528 = f64x8::splat(1.0) / t463;
            let t531 = f64x8::splat(2.0) / f64x8::splat(15.0) * t460 * t451 - f64x8::splat(3.0) / f64x8::splat(35.0) * t460 * t465 + f64x8::splat(8.0) / f64x8::splat(135.0) * t460 * t469 - f64x8::splat(10.0) / f64x8::splat(231.0) * t460 * t473 + f64x8::splat(3.0) / f64x8::splat(91.0) * t460 * t477 - f64x8::splat(7.0) / f64x8::splat(270.0) * t460 * t481 + f64x8::splat(16.0) / f64x8::splat(765.0) * t460 * t485 - f64x8::splat(18.0) / f64x8::splat(1045.0) * t460 * t489 + f64x8::splat(10.0) / f64x8::splat(693.0) * t460 * t493 - f64x8::splat(11.0) / f64x8::splat(897.0) * t460 * t497 + f64x8::splat(24.0) / f64x8::splat(2275.0) * t460 * t501 - f64x8::splat(26.0) / f64x8::splat(2835.0) * t460 * t505 + f64x8::splat(7.0) / f64x8::splat(870.0) * t460 * t509 - f64x8::splat(15.0) / f64x8::splat(2108.0) * t460 * t513 + f64x8::splat(32.0) / f64x8::splat(5049.0) * t460 * t517 - f64x8::splat(34.0) / f64x8::splat(5985.0) * t460 * t521 + f64x8::splat(18.0) / f64x8::splat(3515.0) * t460 * t525 - f64x8::splat(2.0) / f64x8::splat(9.0) * t460 * t528;
            let t532 = ((t190).select(f64x8::splat(0.0), -t459));
            let t535 = f64x8::splat(1.0) / t252;
            let t541 = t249 * t247;
            let t542 = f64x8::splat(1.0) / t541;
            let t543 = t542 * t250;
            let t544 = t535 * t532;
            let t547 = -f64x8::splat(2.0) * t253 * t532 * t247 + f64x8::splat(2.0) * t544 * t543;
            let t550 = -t535 * t251 * t532 + t255 * t532 / f64x8::splat(4.0) + t547 * t247 / f64x8::splat(4.0);
            let t554 = ((t189).select(t531, -f64x8::splat(8.0) / f64x8::splat(3.0) * t550 * t247 - f64x8::splat(8.0) / f64x8::splat(3.0) * t258 * t532));
            let t555 = t554 * t27;
            let t556 = t176 * t555;
            let t560 = ((t148).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t264 * t443 - t449 - f64x8::splat(3.0) / f64x8::splat(8.0) * t556 * t157));
            let tvrho0 = t147 + t267 + (t436 + t560) * t6;
            acc_vrho_0 = tvrho0;
            let t564 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -t7 - t270)));
            let t567 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t564 * t23));
            let t568 = t567 * t5;
            let t572 = t6 * t564 + t18 + f64x8::splat(1.0);
            let t573 = t572 * t326;
            let t575 = t573 * t61 / f64x8::splat(6.0);
            let t576 = ((t70).select(-t575, f64x8::splat(0.0)));
            let t577 = t576 * t286;
            let t579 = t576 * t338;
            let t581 = t576 * t342;
            let t583 = t576 * t346;
            let t585 = t576 * t350;
            let t587 = t576 * t354;
            let t589 = t576 * t358;
            let t591 = t576 * t362;
            let t593 = t576 * t366;
            let t595 = t576 * t370;
            let t597 = t576 * t374;
            let t599 = t576 * t378;
            let t601 = t576 * t382;
            let t603 = t576 * t386;
            let t605 = t576 * t390;
            let t607 = t576 * t394;
            let t609 = t576 * t398;
            let t613 = f64x8::splat(2.0) / f64x8::splat(15.0) * t577 - f64x8::splat(3.0) / f64x8::splat(35.0) * t579 + f64x8::splat(8.0) / f64x8::splat(135.0) * t581 - f64x8::splat(10.0) / f64x8::splat(231.0) * t583 + f64x8::splat(3.0) / f64x8::splat(91.0) * t585 - f64x8::splat(7.0) / f64x8::splat(270.0) * t587 + f64x8::splat(16.0) / f64x8::splat(765.0) * t589 - f64x8::splat(18.0) / f64x8::splat(1045.0) * t591 + f64x8::splat(10.0) / f64x8::splat(693.0) * t593 - f64x8::splat(11.0) / f64x8::splat(897.0) * t595 + f64x8::splat(24.0) / f64x8::splat(2275.0) * t597 - f64x8::splat(26.0) / f64x8::splat(2835.0) * t599 + f64x8::splat(7.0) / f64x8::splat(870.0) * t601 - f64x8::splat(15.0) / f64x8::splat(2108.0) * t603 + f64x8::splat(32.0) / f64x8::splat(5049.0) * t605 - f64x8::splat(34.0) / f64x8::splat(5985.0) * t607 + f64x8::splat(18.0) / f64x8::splat(3515.0) * t609 - f64x8::splat(2.0) / f64x8::splat(9.0) * t576 * t401;
            let t614 = ((t70).select(f64x8::splat(0.0), -t575));
            let t616 = t131 * t614;
            let t622 = t408 * t614;
            let t625 = -f64x8::splat(2.0) * t133 * t614 * t127 + f64x8::splat(2.0) * t622 * t416;
            let t628 = -t408 * t616 + t135 * t614 / f64x8::splat(4.0) + t625 * t127 / f64x8::splat(4.0);
            let t632 = ((t69).select(t613, -f64x8::splat(8.0) / f64x8::splat(3.0) * t628 * t127 - f64x8::splat(8.0) / f64x8::splat(3.0) * t138 * t614));
            let t633 = t632 * t27;
            let t634 = t55 * t633;
            let t638 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t144 * t568 - t284 - f64x8::splat(3.0) / f64x8::splat(8.0) * t634 * t26));
            let t640 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), t7 - t437)));
            let t643 = ((t153).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t640 * t154));
            let t644 = t643 * t5;
            let t649 = f64x8::splat(1.0) / t180 / t179 * param_hyb_omega_0;
            let t651 = f64x8::splat(M_PI) * t186 * t649;
            let t652 = t176 * t176;
            let t653 = f64x8::splat(1.0) / t652;
            let t654 = t653 * t33;
            let t655 = t158 * v_rho1;
            let t657 = f64x8::splat(1.0) / t160 / t655;
            let t658 = t657 * v_sigma2;
            let t662 = t171 * t171;
            let t663 = f64x8::splat(1.0) / t662;
            let t665 = f64x8::splat(1.0) / t159 / t158;
            let t669 = t163 + f64x8::splat(1.0);
            let t670 = ((t669).sqrt());
            let t671 = f64x8::splat(1.0) / t670;
            let t674 = -f64x8::splat(0.0336) * t168 * t665 * t164 - f64x8::splat(0.0336) * t671 * t658;
            let t675 = t674 * t663;
            let t679 = -f64x8::splat(0.002488888888888889) * t172 * t658 * t36 - f64x8::splat(0.0009333333333333333) * t675 * t163 * t36;
            let t685 = t6 * t640 + t151 + f64x8::splat(1.0);
            let t689 = t679 * t654 * t35 * t651 / f64x8::splat(4.0) - t685 * t454 * t182 / f64x8::splat(6.0);
            let t690 = ((t190).select(t689, f64x8::splat(0.0)));
            let t691 = t690 * t451;
            let t693 = t690 * t465;
            let t695 = t690 * t469;
            let t697 = t690 * t473;
            let t699 = t690 * t477;
            let t701 = t690 * t481;
            let t703 = t690 * t485;
            let t705 = t690 * t489;
            let t707 = t690 * t493;
            let t709 = t690 * t497;
            let t711 = t690 * t501;
            let t713 = t690 * t505;
            let t715 = t690 * t509;
            let t717 = t690 * t513;
            let t719 = t690 * t517;
            let t721 = t690 * t521;
            let t723 = t690 * t525;
            let t727 = f64x8::splat(2.0) / f64x8::splat(15.0) * t691 - f64x8::splat(3.0) / f64x8::splat(35.0) * t693 + f64x8::splat(8.0) / f64x8::splat(135.0) * t695 - f64x8::splat(10.0) / f64x8::splat(231.0) * t697 + f64x8::splat(3.0) / f64x8::splat(91.0) * t699 - f64x8::splat(7.0) / f64x8::splat(270.0) * t701 + f64x8::splat(16.0) / f64x8::splat(765.0) * t703 - f64x8::splat(18.0) / f64x8::splat(1045.0) * t705 + f64x8::splat(10.0) / f64x8::splat(693.0) * t707 - f64x8::splat(11.0) / f64x8::splat(897.0) * t709 + f64x8::splat(24.0) / f64x8::splat(2275.0) * t711 - f64x8::splat(26.0) / f64x8::splat(2835.0) * t713 + f64x8::splat(7.0) / f64x8::splat(870.0) * t715 - f64x8::splat(15.0) / f64x8::splat(2108.0) * t717 + f64x8::splat(32.0) / f64x8::splat(5049.0) * t719 - f64x8::splat(34.0) / f64x8::splat(5985.0) * t721 + f64x8::splat(18.0) / f64x8::splat(3515.0) * t723 - f64x8::splat(2.0) / f64x8::splat(9.0) * t690 * t528;
            let t728 = ((t190).select(f64x8::splat(0.0), t689));
            let t730 = t251 * t728;
            let t736 = t535 * t728;
            let t739 = -f64x8::splat(2.0) * t253 * t728 * t247 + f64x8::splat(2.0) * t736 * t543;
            let t742 = -t535 * t730 + t255 * t728 / f64x8::splat(4.0) + t739 * t247 / f64x8::splat(4.0);
            let t746 = ((t189).select(t727, -f64x8::splat(8.0) / f64x8::splat(3.0) * t742 * t247 - f64x8::splat(8.0) / f64x8::splat(3.0) * t258 * t728));
            let t747 = t746 * t27;
            let t748 = t176 * t747;
            let t751 = t679 * t263;
            let t755 = ((t148).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t264 * t644 - t449 - f64x8::splat(3.0) / f64x8::splat(8.0) * t748 * t157 - f64x8::splat(3.0) / f64x8::splat(8.0) * t751 * t157));
            let tvrho1 = t147 + t267 + (t638 + t755) * t6;
            acc_vrho_1 = tvrho1;
            let t762 = f64x8::splat(1.0) / t43;
            let t768 = f64x8::splat(0.0126) * t47 * t45 * t762 + f64x8::splat(0.0126) * t311 * t41;
            let t769 = t768 * t303;
            let t773 = f64x8::splat(0.0009333333333333333) * t51 * t41 * t33 * t35 - f64x8::splat(0.0009333333333333333) * t769 * t42 * t36;
            let t777 = t773 * t294 * t35 * t291 / f64x8::splat(4.0);
            let t778 = ((t70).select(t777, f64x8::splat(0.0)));
            let t779 = t778 * t286;
            let t781 = t778 * t338;
            let t783 = t778 * t342;
            let t785 = t778 * t346;
            let t787 = t778 * t350;
            let t789 = t778 * t354;
            let t791 = t778 * t358;
            let t793 = t778 * t362;
            let t795 = t778 * t366;
            let t797 = t778 * t370;
            let t799 = t778 * t374;
            let t801 = t778 * t378;
            let t803 = t778 * t382;
            let t805 = t778 * t386;
            let t807 = t778 * t390;
            let t809 = t778 * t394;
            let t811 = t778 * t398;
            let t815 = f64x8::splat(2.0) / f64x8::splat(15.0) * t779 - f64x8::splat(3.0) / f64x8::splat(35.0) * t781 + f64x8::splat(8.0) / f64x8::splat(135.0) * t783 - f64x8::splat(10.0) / f64x8::splat(231.0) * t785 + f64x8::splat(3.0) / f64x8::splat(91.0) * t787 - f64x8::splat(7.0) / f64x8::splat(270.0) * t789 + f64x8::splat(16.0) / f64x8::splat(765.0) * t791 - f64x8::splat(18.0) / f64x8::splat(1045.0) * t793 + f64x8::splat(10.0) / f64x8::splat(693.0) * t795 - f64x8::splat(11.0) / f64x8::splat(897.0) * t797 + f64x8::splat(24.0) / f64x8::splat(2275.0) * t799 - f64x8::splat(26.0) / f64x8::splat(2835.0) * t801 + f64x8::splat(7.0) / f64x8::splat(870.0) * t803 - f64x8::splat(15.0) / f64x8::splat(2108.0) * t805 + f64x8::splat(32.0) / f64x8::splat(5049.0) * t807 - f64x8::splat(34.0) / f64x8::splat(5985.0) * t809 + f64x8::splat(18.0) / f64x8::splat(3515.0) * t811 - f64x8::splat(2.0) / f64x8::splat(9.0) * t778 * t401;
            let t816 = ((t70).select(f64x8::splat(0.0), t777));
            let t818 = t131 * t816;
            let t824 = t408 * t816;
            let t827 = -f64x8::splat(2.0) * t133 * t816 * t127 + f64x8::splat(2.0) * t824 * t416;
            let t830 = -t408 * t818 + t135 * t816 / f64x8::splat(4.0) + t827 * t127 / f64x8::splat(4.0);
            let t834 = ((t69).select(t815, -f64x8::splat(8.0) / f64x8::splat(3.0) * t830 * t127 - f64x8::splat(8.0) / f64x8::splat(3.0) * t138 * t816));
            let t835 = t834 * t27;
            let t836 = t55 * t835;
            let t838 = t773 * t143;
            let t842 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t836 * t26 - f64x8::splat(3.0) / f64x8::splat(8.0) * t838 * t26));
            let tvsigma0 = t842 * t6;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t847 = f64x8::splat(1.0) / t164;
            let t853 = f64x8::splat(0.0126) * t168 * t166 * t847 + f64x8::splat(0.0126) * t671 * t162;
            let t854 = t853 * t663;
            let t858 = f64x8::splat(0.0009333333333333333) * t172 * t162 * t33 * t35 - f64x8::splat(0.0009333333333333333) * t854 * t163 * t36;
            let t862 = t858 * t654 * t35 * t651 / f64x8::splat(4.0);
            let t863 = ((t190).select(t862, f64x8::splat(0.0)));
            let t864 = t863 * t451;
            let t866 = t863 * t465;
            let t868 = t863 * t469;
            let t870 = t863 * t473;
            let t872 = t863 * t477;
            let t874 = t863 * t481;
            let t876 = t863 * t485;
            let t878 = t863 * t489;
            let t880 = t863 * t493;
            let t882 = t863 * t497;
            let t884 = t863 * t501;
            let t886 = t863 * t505;
            let t888 = t863 * t509;
            let t890 = t863 * t513;
            let t892 = t863 * t517;
            let t894 = t863 * t521;
            let t896 = t863 * t525;
            let t900 = f64x8::splat(2.0) / f64x8::splat(15.0) * t864 - f64x8::splat(3.0) / f64x8::splat(35.0) * t866 + f64x8::splat(8.0) / f64x8::splat(135.0) * t868 - f64x8::splat(10.0) / f64x8::splat(231.0) * t870 + f64x8::splat(3.0) / f64x8::splat(91.0) * t872 - f64x8::splat(7.0) / f64x8::splat(270.0) * t874 + f64x8::splat(16.0) / f64x8::splat(765.0) * t876 - f64x8::splat(18.0) / f64x8::splat(1045.0) * t878 + f64x8::splat(10.0) / f64x8::splat(693.0) * t880 - f64x8::splat(11.0) / f64x8::splat(897.0) * t882 + f64x8::splat(24.0) / f64x8::splat(2275.0) * t884 - f64x8::splat(26.0) / f64x8::splat(2835.0) * t886 + f64x8::splat(7.0) / f64x8::splat(870.0) * t888 - f64x8::splat(15.0) / f64x8::splat(2108.0) * t890 + f64x8::splat(32.0) / f64x8::splat(5049.0) * t892 - f64x8::splat(34.0) / f64x8::splat(5985.0) * t894 + f64x8::splat(18.0) / f64x8::splat(3515.0) * t896 - f64x8::splat(2.0) / f64x8::splat(9.0) * t863 * t528;
            let t901 = ((t190).select(f64x8::splat(0.0), t862));
            let t903 = t251 * t901;
            let t909 = t535 * t901;
            let t912 = -f64x8::splat(2.0) * t253 * t901 * t247 + f64x8::splat(2.0) * t909 * t543;
            let t915 = -t535 * t903 + t255 * t901 / f64x8::splat(4.0) + t912 * t247 / f64x8::splat(4.0);
            let t919 = ((t189).select(t900, -f64x8::splat(8.0) / f64x8::splat(3.0) * t915 * t247 - f64x8::splat(8.0) / f64x8::splat(3.0) * t258 * t901));
            let t920 = t919 * t27;
            let t921 = t176 * t920;
            let t923 = t858 * t263;
            let t927 = ((t148).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t921 * t157 - f64x8::splat(3.0) / f64x8::splat(8.0) * t923 * t157));
            let tvsigma2 = t927 * t6;
            acc_vsigma_2 = tvsigma2;
        }
        store_add(zk, ip, m, acc_zk);
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(vsigma, ip, m, 3, 0, acc_vsigma_0);
        store_strided(vsigma, ip, m, 3, 1, acc_vsigma_1);
        store_strided(vsigma, ip, m, 3, 2, acc_vsigma_2);
        ip += 8;
    }
}
