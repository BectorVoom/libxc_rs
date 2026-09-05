//! GGA_X_PBEPOW kxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_pbepow.c`
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
pub fn gga_x_pbepow_kxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    v3rho3: &mut [f64],
    v3rho2sigma: &mut [f64],
    v3rhosigma2: &mut [f64],
    v3sigma3: &mut [f64],
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
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        let mut acc_vsigma_0 = V_ZERO;
        let mut acc_vsigma_1 = V_ZERO;
        let mut acc_vsigma_2 = V_ZERO;
        let mut acc_v2rho2_0 = V_ZERO;
        let mut acc_v2rho2_1 = V_ZERO;
        let mut acc_v2rho2_2 = V_ZERO;
        let mut acc_v2rhosigma_0 = V_ZERO;
        let mut acc_v2rhosigma_1 = V_ZERO;
        let mut acc_v2rhosigma_2 = V_ZERO;
        let mut acc_v2rhosigma_3 = V_ZERO;
        let mut acc_v2rhosigma_4 = V_ZERO;
        let mut acc_v2rhosigma_5 = V_ZERO;
        let mut acc_v2sigma2_0 = V_ZERO;
        let mut acc_v2sigma2_1 = V_ZERO;
        let mut acc_v2sigma2_2 = V_ZERO;
        let mut acc_v2sigma2_3 = V_ZERO;
        let mut acc_v2sigma2_4 = V_ZERO;
        let mut acc_v2sigma2_5 = V_ZERO;
        let mut acc_v3rho3_0 = V_ZERO;
        let mut acc_v3rho3_1 = V_ZERO;
        let mut acc_v3rho3_2 = V_ZERO;
        let mut acc_v3rho3_3 = V_ZERO;
        let mut acc_v3rho2sigma_0 = V_ZERO;
        let mut acc_v3rho2sigma_1 = V_ZERO;
        let mut acc_v3rho2sigma_2 = V_ZERO;
        let mut acc_v3rho2sigma_3 = V_ZERO;
        let mut acc_v3rho2sigma_4 = V_ZERO;
        let mut acc_v3rho2sigma_5 = V_ZERO;
        let mut acc_v3rho2sigma_6 = V_ZERO;
        let mut acc_v3rho2sigma_7 = V_ZERO;
        let mut acc_v3rho2sigma_8 = V_ZERO;
        let mut acc_v3rhosigma2_0 = V_ZERO;
        let mut acc_v3rhosigma2_1 = V_ZERO;
        let mut acc_v3rhosigma2_2 = V_ZERO;
        let mut acc_v3rhosigma2_3 = V_ZERO;
        let mut acc_v3rhosigma2_4 = V_ZERO;
        let mut acc_v3rhosigma2_5 = V_ZERO;
        let mut acc_v3rhosigma2_6 = V_ZERO;
        let mut acc_v3rhosigma2_7 = V_ZERO;
        let mut acc_v3rhosigma2_8 = V_ZERO;
        let mut acc_v3rhosigma2_9 = V_ZERO;
        let mut acc_v3rhosigma2_10 = V_ZERO;
        let mut acc_v3rhosigma2_11 = V_ZERO;
        let mut acc_v3sigma3_0 = V_ZERO;
        let mut acc_v3sigma3_1 = V_ZERO;
        let mut acc_v3sigma3_2 = V_ZERO;
        let mut acc_v3sigma3_3 = V_ZERO;
        let mut acc_v3sigma3_4 = V_ZERO;
        let mut acc_v3sigma3_5 = V_ZERO;
        let mut acc_v3sigma3_6 = V_ZERO;
        let mut acc_v3sigma3_7 = V_ZERO;
        let mut acc_v3sigma3_8 = V_ZERO;
        let mut acc_v3sigma3_9 = V_ZERO;
        {
            let t1 = (v_rho0).simd_le(dens_threshold);
            let t2 = f64x8::splat(M_CBRT3);
            let t3 = f64x8::splat(M_CBRTPI);
            let t5 = t2 / t3;
            let t6 = v_rho0 + v_rho1;
            let t7 = f64x8::splat(1.0) / t6;
            let t10 = (f64x8::splat(2.0) * v_rho0 * t7).simd_le(zeta_threshold);
            let t11 = zeta_threshold - f64x8::splat(1.0);
            let t14 = (f64x8::splat(2.0) * v_rho1 * t7).simd_le(zeta_threshold);
            let t15 = -t11;
            let t16 = v_rho0 - v_rho1;
            let t18 = ((t10).select(t11, (t14).select(t15, t16 * t7)));
            let t19 = f64x8::splat(1.0) + t18;
            let t20 = (t19).simd_le(zeta_threshold);
            let t21 = (simd::cbrt(zeta_threshold));
            let t22 = t21 * zeta_threshold;
            let t23 = (simd::cbrt(t19));
            let t25 = ((t20).select(t22, t23 * t19));
            let t26 = (simd::cbrt(t6));
            let t27 = t25 * t26;
            let t28 = f64x8::splat(M_CBRT6);
            let t29 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t30 = (simd::cbrt(t29));
            let t31 = t30 * t30;
            let t32 = f64x8::splat(1.0) / t31;
            let t33 = t28 * t32;
            let t34 = v_rho0 * v_rho0;
            let t35 = (simd::cbrt(v_rho0));
            let t36 = t35 * t35;
            let t38 = f64x8::splat(1.0) / t36 / t34;
            let t39 = v_sigma0 * t38;
            let t40 = t33 * t39;
            let t42 = f64x8::splat(0.9146457198521546) * t40 + f64x8::splat(0.804);
            let t43 = f64x8::splat(1.0) / t42;
            let t45 = t33 * t39 * t43;
            let t46 = (simd::pow(t45, f64x8::splat(100.0)));
            let t48 = f64x8::splat(0.0001334414156799501) * t46 - f64x8::splat(1.0);
            let t52 = f64x8::splat(1.0) - f64x8::splat(0.009146457198521547) * t33 * t39 * t48;
            let t56 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t52));
            let t57 = (v_rho1).simd_le(dens_threshold);
            let t58 = -t16;
            let t60 = ((t14).select(t11, (t10).select(t15, t58 * t7)));
            let t61 = f64x8::splat(1.0) + t60;
            let t62 = (t61).simd_le(zeta_threshold);
            let t63 = (simd::cbrt(t61));
            let t65 = ((t62).select(t22, t63 * t61));
            let t66 = t65 * t26;
            let t67 = v_rho1 * v_rho1;
            let t68 = (simd::cbrt(v_rho1));
            let t69 = t68 * t68;
            let t71 = f64x8::splat(1.0) / t69 / t67;
            let t72 = v_sigma2 * t71;
            let t73 = t33 * t72;
            let t75 = f64x8::splat(0.9146457198521546) * t73 + f64x8::splat(0.804);
            let t76 = f64x8::splat(1.0) / t75;
            let t78 = t33 * t72 * t76;
            let t79 = (simd::pow(t78, f64x8::splat(100.0)));
            let t81 = f64x8::splat(0.0001334414156799501) * t79 - f64x8::splat(1.0);
            let t85 = f64x8::splat(1.0) - f64x8::splat(0.009146457198521547) * t33 * t72 * t81;
            let t89 = ((t57).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t66 * t85));
            let tzk0 = t56 + t89;
            acc_zk = tzk0;
            let t90 = t6 * t6;
            let t91 = f64x8::splat(1.0) / t90;
            let t92 = t16 * t91;
            let t94 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), t7 - t92)));
            let t97 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t94));
            let t98 = t97 * t26;
            let t102 = t26 * t26;
            let t103 = f64x8::splat(1.0) / t102;
            let t104 = t25 * t103;
            let t107 = t5 * t104 * t52 / f64x8::splat(8.0);
            let t108 = t34 * v_rho0;
            let t110 = f64x8::splat(1.0) / t36 / t108;
            let t111 = v_sigma0 * t110;
            let t115 = t33 * v_sigma0;
            let t116 = (simd::pow(t45, f64x8::splat(99.0)));
            let t117 = t38 * t116;
            let t121 = t28 * t28;
            let t123 = f64x8::splat(1.0) / t30 / t29;
            let t124 = t121 * t123;
            let t125 = v_sigma0 * v_sigma0;
            let t126 = t34 * t34;
            let t127 = t126 * t34;
            let t129 = f64x8::splat(1.0) / t35 / t127;
            let t131 = t42 * t42;
            let t132 = f64x8::splat(1.0) / t131;
            let t136 = -f64x8::splat(8.0) / f64x8::splat(3.0) * t33 * t111 * t43 + f64x8::splat(2.4390552529390788) * t124 * t125 * t129 * t132;
            let t137 = t117 * t136;
            let t140 = f64x8::splat(0.024390552529390788) * t33 * t111 * t48 - f64x8::splat(0.00012205161970267855) * t115 * t137;
            let t145 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t98 * t52 - t107 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t140));
            let t146 = t58 * t91;
            let t148 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -t7 - t146)));
            let t151 = ((t62).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t63 * t148));
            let t152 = t151 * t26;
            let t156 = t65 * t103;
            let t159 = t5 * t156 * t85 / f64x8::splat(8.0);
            let t161 = ((t57).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t152 * t85 - t159));
            let tvrho0 = t56 + t89 + t6 * (t145 + t161);
            acc_vrho_0 = tvrho0;
            let t165 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -t7 - t92)));
            let t168 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t165));
            let t169 = t168 * t26;
            let t174 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t169 * t52 - t107));
            let t176 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), t7 - t146)));
            let t179 = ((t62).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t63 * t176));
            let t180 = t179 * t26;
            let t184 = t67 * v_rho1;
            let t186 = f64x8::splat(1.0) / t69 / t184;
            let t187 = v_sigma2 * t186;
            let t191 = t33 * v_sigma2;
            let t192 = (simd::pow(t78, f64x8::splat(99.0)));
            let t193 = t71 * t192;
            let t197 = v_sigma2 * v_sigma2;
            let t198 = t67 * t67;
            let t199 = t198 * t67;
            let t201 = f64x8::splat(1.0) / t68 / t199;
            let t203 = t75 * t75;
            let t204 = f64x8::splat(1.0) / t203;
            let t208 = -f64x8::splat(8.0) / f64x8::splat(3.0) * t33 * t187 * t76 + f64x8::splat(2.4390552529390788) * t124 * t197 * t201 * t204;
            let t209 = t193 * t208;
            let t212 = f64x8::splat(0.024390552529390788) * t33 * t187 * t81 - f64x8::splat(0.00012205161970267855) * t191 * t209;
            let t217 = ((t57).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t180 * t85 - t159 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t66 * t212));
            let tvrho1 = t56 + t89 + t6 * (t174 + t217);
            acc_vrho_1 = tvrho1;
            let t225 = t126 * v_rho0;
            let t227 = f64x8::splat(1.0) / t35 / t225;
            let t232 = t33 * t38 * t43 - f64x8::splat(0.9146457198521546) * t124 * v_sigma0 * t227 * t132;
            let t233 = t117 * t232;
            let t236 = -f64x8::splat(0.009146457198521547) * t33 * t38 * t48 - f64x8::splat(0.00012205161970267855) * t115 * t233;
            let t240 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t236));
            let tvsigma0 = t6 * t240;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t246 = t198 * v_rho1;
            let t248 = f64x8::splat(1.0) / t68 / t246;
            let t253 = t33 * t71 * t76 - f64x8::splat(0.9146457198521546) * t124 * v_sigma2 * t248 * t204;
            let t254 = t193 * t253;
            let t257 = -f64x8::splat(0.009146457198521547) * t33 * t71 * t81 - f64x8::splat(0.00012205161970267855) * t191 * t254;
            let t261 = ((t57).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t66 * t257));
            let tvsigma2 = t6 * t261;
            acc_vsigma_2 = tvsigma2;
            let t264 = t23 * t23;
            let t265 = f64x8::splat(1.0) / t264;
            let t266 = t94 * t94;
            let t269 = t90 * t6;
            let t270 = f64x8::splat(1.0) / t269;
            let t271 = t16 * t270;
            let t274 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -f64x8::splat(2.0) * t91 + f64x8::splat(2.0) * t271)));
            let t278 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t265 * t266 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t274));
            let t279 = t278 * t26;
            let t283 = t97 * t103;
            let t285 = t5 * t283 * t52;
            let t291 = f64x8::splat(1.0) / t102 / t6;
            let t292 = t25 * t291;
            let t295 = t5 * t292 * t52 / f64x8::splat(12.0);
            let t297 = t5 * t104 * t140;
            let t300 = f64x8::splat(1.0) / t36 / t126;
            let t301 = v_sigma0 * t300;
            let t305 = t110 * t116;
            let t306 = t305 * t136;
            let t309 = (simd::pow(t45, f64x8::splat(98.0)));
            let t310 = t38 * t309;
            let t311 = t136 * t136;
            let t312 = t310 * t311;
            let t318 = t126 * t108;
            let t320 = f64x8::splat(1.0) / t35 / t318;
            let t325 = t125 * v_sigma0;
            let t326 = t126 * t126;
            let t327 = t326 * t34;
            let t328 = f64x8::splat(1.0) / t327;
            let t331 = f64x8::splat(1.0) / t131 / t42;
            let t334 = f64x8::splat(88.0) / f64x8::splat(9.0) * t33 * t301 * t43 - f64x8::splat(21.95149727645171) * t124 * t125 * t320 * t132 + f64x8::splat(0.7328667741880203) * t325 * t328 * t331;
            let t335 = t117 * t334;
            let t338 = -f64x8::splat(0.08943202594109956) * t33 * t301 * t48 + f64x8::splat(0.0006509419717476189) * t115 * t306 - f64x8::splat(0.012083110350565177) * t115 * t312 - f64x8::splat(0.00012205161970267855) * t115 * t335;
            let t343 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t279 * t52 - t285 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t5 * t98 * t140 + t295 - t297 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t338));
            let t344 = t63 * t63;
            let t345 = f64x8::splat(1.0) / t344;
            let t346 = t148 * t148;
            let t349 = t58 * t270;
            let t352 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), f64x8::splat(2.0) * t91 + f64x8::splat(2.0) * t349)));
            let t356 = ((t62).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t345 * t346 + f64x8::splat(4.0) / f64x8::splat(3.0) * t63 * t352));
            let t357 = t356 * t26;
            let t361 = t151 * t103;
            let t363 = t5 * t361 * t85;
            let t365 = t65 * t291;
            let t368 = t5 * t365 * t85 / f64x8::splat(12.0);
            let t370 = ((t57).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t357 * t85 - t363 / f64x8::splat(4.0) + t368));
            let tv2rho20 = f64x8::splat(2.0) * t145 + f64x8::splat(2.0) * t161 + t6 * (t343 + t370);
            acc_v2rho2_0 = tv2rho20;
            let t373 = t265 * t165;
            let t377 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), f64x8::splat(2.0) * t271)));
            let t381 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t373 * t94 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t377));
            let t382 = t381 * t26;
            let t386 = t168 * t103;
            let t388 = t5 * t386 * t52;
            let t396 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t382 * t52 - t388 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t169 * t140 - t285 / f64x8::splat(8.0) + t295 - t297 / f64x8::splat(8.0)));
            let t397 = t345 * t176;
            let t401 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), f64x8::splat(2.0) * t349)));
            let t405 = ((t62).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t397 * t148 + f64x8::splat(4.0) / f64x8::splat(3.0) * t63 * t401));
            let t406 = t405 * t26;
            let t410 = t179 * t103;
            let t412 = t5 * t410 * t85;
            let t419 = t5 * t156 * t212;
            let t422 = ((t57).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t406 * t85 - t412 / f64x8::splat(8.0) - t363 / f64x8::splat(8.0) + t368 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t152 * t212 - t419 / f64x8::splat(8.0)));
            let tv2rho21 = t145 + t161 + t174 + t217 + t6 * (t396 + t422);
            acc_v2rho2_1 = tv2rho21;
            let t427 = t165 * t165;
            let t432 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), f64x8::splat(2.0) * t91 + f64x8::splat(2.0) * t271)));
            let t436 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t265 * t427 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t432));
            let t437 = t436 * t26;
            let t443 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t437 * t52 - t388 / f64x8::splat(4.0) + t295));
            let t444 = t176 * t176;
            let t449 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -f64x8::splat(2.0) * t91 + f64x8::splat(2.0) * t349)));
            let t453 = ((t62).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t345 * t444 + f64x8::splat(4.0) / f64x8::splat(3.0) * t63 * t449));
            let t454 = t453 * t26;
            let t464 = f64x8::splat(1.0) / t69 / t198;
            let t465 = v_sigma2 * t464;
            let t469 = t186 * t192;
            let t470 = t469 * t208;
            let t473 = (simd::pow(t78, f64x8::splat(98.0)));
            let t474 = t71 * t473;
            let t475 = t208 * t208;
            let t476 = t474 * t475;
            let t482 = t198 * t184;
            let t484 = f64x8::splat(1.0) / t68 / t482;
            let t489 = t197 * v_sigma2;
            let t490 = t198 * t198;
            let t491 = t490 * t67;
            let t492 = f64x8::splat(1.0) / t491;
            let t495 = f64x8::splat(1.0) / t203 / t75;
            let t498 = f64x8::splat(88.0) / f64x8::splat(9.0) * t33 * t465 * t76 - f64x8::splat(21.95149727645171) * t124 * t197 * t484 * t204 + f64x8::splat(0.7328667741880203) * t489 * t492 * t495;
            let t499 = t193 * t498;
            let t502 = -f64x8::splat(0.08943202594109956) * t33 * t465 * t81 + f64x8::splat(0.0006509419717476189) * t191 * t470 - f64x8::splat(0.012083110350565177) * t191 * t476 - f64x8::splat(0.00012205161970267855) * t191 * t499;
            let t507 = ((t57).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t454 * t85 - t412 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t5 * t180 * t212 + t368 - t419 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t66 * t502));
            let tv2rho22 = f64x8::splat(2.0) * t174 + f64x8::splat(2.0) * t217 + t6 * (t443 + t507);
            acc_v2rho2_2 = tv2rho22;
            let t515 = t5 * t104 * t236 / f64x8::splat(8.0);
            let t521 = t305 * t232;
            let t524 = t232 * t136;
            let t531 = t129 * t132;
            let t535 = t326 * v_rho0;
            let t536 = f64x8::splat(1.0) / t535;
            let t540 = -f64x8::splat(8.0) / f64x8::splat(3.0) * t33 * t110 * t43 + f64x8::splat(7.317165758817237) * t124 * t531 * v_sigma0 - f64x8::splat(0.2748250403205076) * t125 * t536 * t331;
            let t541 = t117 * t540;
            let t544 = f64x8::splat(0.024390552529390788) * t33 * t110 * t48 - f64x8::splat(0.00012205161970267855) * t33 * t137 + f64x8::splat(0.00032547098587380947) * t115 * t521 - f64x8::splat(0.012083110350565177) * t115 * t310 * t524 - f64x8::splat(0.00012205161970267855) * t115 * t541;
            let t549 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t98 * t236 - t515 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t544));
            let tv2rhosigma0 = t6 * t549 + t240;
            acc_v2rhosigma_0 = tv2rhosigma0;
            let tv2rhosigma1 = f64x8::splat(0.0);
            acc_v2rhosigma_1 = tv2rhosigma1;
            let t556 = t5 * t156 * t257 / f64x8::splat(8.0);
            let t558 = ((t57).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t152 * t257 - t556));
            let tv2rhosigma2 = t6 * t558 + t261;
            acc_v2rhosigma_2 = tv2rhosigma2;
            let t564 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t169 * t236 - t515));
            let tv2rhosigma3 = t6 * t564 + t240;
            acc_v2rhosigma_3 = tv2rhosigma3;
            let tv2rhosigma4 = f64x8::splat(0.0);
            acc_v2rhosigma_4 = tv2rhosigma4;
            let t574 = t469 * t253;
            let t577 = t253 * t208;
            let t584 = t201 * t204;
            let t588 = t490 * v_rho1;
            let t589 = f64x8::splat(1.0) / t588;
            let t593 = -f64x8::splat(8.0) / f64x8::splat(3.0) * t33 * t186 * t76 + f64x8::splat(7.317165758817237) * t124 * t584 * v_sigma2 - f64x8::splat(0.2748250403205076) * t197 * t589 * t495;
            let t594 = t193 * t593;
            let t597 = f64x8::splat(0.024390552529390788) * t33 * t186 * t81 - f64x8::splat(0.00012205161970267855) * t33 * t209 + f64x8::splat(0.00032547098587380947) * t191 * t574 - f64x8::splat(0.012083110350565177) * t191 * t474 * t577 - f64x8::splat(0.00012205161970267855) * t191 * t594;
            let t602 = ((t57).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t180 * t257 - t556 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t66 * t597));
            let tv2rhosigma5 = t6 * t602 + t261;
            acc_v2rhosigma_5 = tv2rhosigma5;
            let t606 = t232 * t232;
            let t607 = t310 * t606;
            let t613 = f64x8::splat(1.0) / t326;
            let t617 = -f64x8::splat(1.8292914397043092) * t124 * t227 * t132 + f64x8::splat(0.10305939012019034) * v_sigma0 * t613 * t331;
            let t618 = t117 * t617;
            let t621 = -f64x8::splat(0.0002441032394053571) * t33 * t233 - f64x8::splat(0.012083110350565177) * t115 * t607 - f64x8::splat(0.00012205161970267855) * t115 * t618;
            let t625 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t621));
            let tv2sigma20 = t6 * t625;
            acc_v2sigma2_0 = tv2sigma20;
            let tv2sigma21 = f64x8::splat(0.0);
            acc_v2sigma2_1 = tv2sigma21;
            let tv2sigma22 = f64x8::splat(0.0);
            acc_v2sigma2_2 = tv2sigma22;
            let tv2sigma23 = f64x8::splat(0.0);
            acc_v2sigma2_3 = tv2sigma23;
            let tv2sigma24 = f64x8::splat(0.0);
            acc_v2sigma2_4 = tv2sigma24;
            let t628 = t253 * t253;
            let t629 = t474 * t628;
            let t635 = f64x8::splat(1.0) / t490;
            let t639 = -f64x8::splat(1.8292914397043092) * t124 * t248 * t204 + f64x8::splat(0.10305939012019034) * v_sigma2 * t635 * t495;
            let t640 = t193 * t639;
            let t643 = -f64x8::splat(0.0002441032394053571) * t33 * t254 - f64x8::splat(0.012083110350565177) * t191 * t629 - f64x8::splat(0.00012205161970267855) * t191 * t640;
            let t647 = ((t57).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t66 * t643));
            let tv2sigma25 = t6 * t647;
            acc_v2sigma2_5 = tv2sigma25;
            let t651 = f64x8::splat(1.0) / t264 / t19;
            let t652 = t266 * t94;
            let t655 = t265 * t94;
            let t658 = t90 * t90;
            let t659 = f64x8::splat(1.0) / t658;
            let t660 = t16 * t659;
            let t663 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), f64x8::splat(6.0) * t270 - f64x8::splat(6.0) * t660)));
            let t667 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t651 * t652 + f64x8::splat(4.0) / f64x8::splat(3.0) * t655 * t274 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t663));
            let t668 = t667 * t26;
            let t672 = t278 * t103;
            let t674 = t5 * t672 * t52;
            let t679 = t97 * t291;
            let t681 = t5 * t679 * t52;
            let t684 = t5 * t283 * t140;
            let t690 = f64x8::splat(1.0) / t102 / t90;
            let t691 = t25 * t690;
            let t694 = f64x8::splat(5.0) / f64x8::splat(36.0) * t5 * t691 * t52;
            let t696 = t5 * t292 * t140;
            let t699 = t5 * t104 * t338;
            let t702 = f64x8::splat(1.0) / t36 / t225;
            let t703 = v_sigma0 * t702;
            let t707 = t300 * t116;
            let t708 = t707 * t136;
            let t711 = t110 * t309;
            let t712 = t711 * t311;
            let t715 = t305 * t334;
            let t718 = (simd::pow(t45, f64x8::splat(97.0)));
            let t719 = t38 * t718;
            let t720 = t311 * t136;
            let t721 = t719 * t720;
            let t724 = t136 * t334;
            let t732 = f64x8::splat(1.0) / t35 / t326;
            let t737 = t326 * t108;
            let t738 = f64x8::splat(1.0) / t737;
            let t742 = t125 * t125;
            let t743 = t326 * t225;
            let t745 = f64x8::splat(1.0) / t36 / t743;
            let t747 = t131 * t131;
            let t748 = f64x8::splat(1.0) / t747;
            let t750 = t748 * t28 * t32;
            let t753 = -f64x8::splat(1232.0) / f64x8::splat(27.0) * t33 * t703 * t43 + f64x8::splat(184.82618694493908) * t124 * t125 * t732 * t132 - f64x8::splat(13.924468709572384) * t325 * t738 * t331 + f64x8::splat(5.362507665863426) * t742 * t745 * t750;
            let t754 = t117 * t753;
            let t757 = f64x8::splat(0.41734945439179794) * t33 * t703 * t48 - f64x8::splat(0.003580180844611904) * t115 * t708 + f64x8::splat(0.09666488280452142) * t115 * t712 + f64x8::splat(0.0009764129576214284) * t115 * t715 - f64x8::splat(1.1841448143553872) * t115 * t721 - f64x8::splat(0.036249331051695526) * t115 * t310 * t724 - f64x8::splat(0.00012205161970267855) * t115 * t754;
            let t762 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t668 * t52 - f64x8::splat(3.0) / f64x8::splat(8.0) * t674 - f64x8::splat(9.0) / f64x8::splat(8.0) * t5 * t279 * t140 + t681 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t684 - f64x8::splat(9.0) / f64x8::splat(8.0) * t5 * t98 * t338 - t694 + t696 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t699 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t757));
            let t764 = f64x8::splat(1.0) / t344 / t61;
            let t765 = t346 * t148;
            let t768 = t345 * t148;
            let t771 = t58 * t659;
            let t774 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -f64x8::splat(6.0) * t270 - f64x8::splat(6.0) * t771)));
            let t778 = ((t62).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t764 * t765 + f64x8::splat(4.0) / f64x8::splat(3.0) * t768 * t352 + f64x8::splat(4.0) / f64x8::splat(3.0) * t63 * t774));
            let t779 = t778 * t26;
            let t783 = t356 * t103;
            let t785 = t5 * t783 * t85;
            let t787 = t151 * t291;
            let t789 = t5 * t787 * t85;
            let t791 = t65 * t690;
            let t794 = f64x8::splat(5.0) / f64x8::splat(36.0) * t5 * t791 * t85;
            let t796 = ((t57).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t779 * t85 - f64x8::splat(3.0) / f64x8::splat(8.0) * t785 + t789 / f64x8::splat(4.0) - t794));
            let tv3rho30 = f64x8::splat(3.0) * t343 + f64x8::splat(3.0) * t370 + t6 * (t762 + t796);
            acc_v3rho3_0 = tv3rho30;
            let t799 = f64x8::splat(2.0) * t396;
            let t800 = f64x8::splat(2.0) * t422;
            let t801 = t651 * t165;
            let t804 = t265 * t377;
            let t809 = f64x8::splat(2.0) * t270;
            let t810 = f64x8::splat(6.0) * t660;
            let t812 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), t809 - t810)));
            let t816 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t801 * t266 + f64x8::splat(8.0) / f64x8::splat(9.0) * t804 * t94 + f64x8::splat(4.0) / f64x8::splat(9.0) * t373 * t274 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t812));
            let t817 = t816 * t26;
            let t821 = t381 * t103;
            let t824 = t5 * t821 * t52 / f64x8::splat(4.0);
            let t828 = t168 * t291;
            let t830 = t5 * t828 * t52;
            let t834 = t5 * t386 * t140 / f64x8::splat(4.0);
            let t843 = -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t817 * t52 - t824 - f64x8::splat(3.0) / f64x8::splat(4.0) * t5 * t382 * t140 + t830 / f64x8::splat(12.0) - t834 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t169 * t338 - t674 / f64x8::splat(8.0) + t681 / f64x8::splat(6.0) - t684 / f64x8::splat(4.0) - t694 + t696 / f64x8::splat(6.0) - t699 / f64x8::splat(8.0);
            let t844 = ((t1).select(f64x8::splat(0.0), t843));
            let t845 = t764 * t176;
            let t848 = t345 * t401;
            let t853 = f64x8::splat(6.0) * t771;
            let t855 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -t809 - t853)));
            let t859 = ((t62).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t845 * t346 + f64x8::splat(8.0) / f64x8::splat(9.0) * t848 * t148 + f64x8::splat(4.0) / f64x8::splat(9.0) * t397 * t352 + f64x8::splat(4.0) / f64x8::splat(3.0) * t63 * t855));
            let t860 = t859 * t26;
            let t864 = t405 * t103;
            let t867 = t5 * t864 * t85 / f64x8::splat(4.0);
            let t868 = t179 * t291;
            let t870 = t5 * t868 * t85;
            let t879 = t5 * t361 * t212 / f64x8::splat(4.0);
            let t881 = t5 * t365 * t212;
            let t884 = ((t57).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t860 * t85 - t867 + t870 / f64x8::splat(12.0) - t785 / f64x8::splat(8.0) + t789 / f64x8::splat(6.0) - t794 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t357 * t212 - t879 + t881 / f64x8::splat(12.0)));
            let tv3rho31 = t343 + t370 + t799 + t800 + t6 * (t844 + t884);
            acc_v3rho3_1 = tv3rho31;
            let t887 = t651 * t427;
            let t892 = t265 * t432;
            let t896 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -t809 - t810)));
            let t900 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t887 * t94 + f64x8::splat(8.0) / f64x8::splat(9.0) * t373 * t377 + f64x8::splat(4.0) / f64x8::splat(9.0) * t892 * t94 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t896));
            let t901 = t900 * t26;
            let t905 = t436 * t103;
            let t907 = t5 * t905 * t52;
            let t916 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t901 * t52 - t907 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t437 * t140 - t824 + t830 / f64x8::splat(6.0) - t834 + t681 / f64x8::splat(12.0) - t694 + t696 / f64x8::splat(12.0)));
            let t917 = t764 * t444;
            let t922 = t345 * t449;
            let t926 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), t809 - t853)));
            let t930 = ((t62).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t917 * t148 + f64x8::splat(8.0) / f64x8::splat(9.0) * t397 * t401 + f64x8::splat(4.0) / f64x8::splat(9.0) * t922 * t148 + f64x8::splat(4.0) / f64x8::splat(3.0) * t63 * t926));
            let t931 = t930 * t26;
            let t935 = t453 * t103;
            let t937 = t5 * t935 * t85;
            let t944 = t5 * t410 * t212;
            let t952 = t5 * t156 * t502;
            let t954 = -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t931 * t85 - t937 / f64x8::splat(8.0) - t867 + t870 / f64x8::splat(6.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t5 * t406 * t212 - t944 / f64x8::splat(4.0) + t789 / f64x8::splat(12.0) - t794 - t879 + t881 / f64x8::splat(6.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t152 * t502 - t952 / f64x8::splat(8.0);
            let t955 = ((t57).select(f64x8::splat(0.0), t954));
            let tv3rho32 = t799 + t800 + t443 + t507 + t6 * (t916 + t955);
            acc_v3rho3_2 = tv3rho32;
            let t960 = t427 * t165;
            let t967 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -f64x8::splat(6.0) * t270 - f64x8::splat(6.0) * t660)));
            let t971 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t651 * t960 + f64x8::splat(4.0) / f64x8::splat(3.0) * t373 * t432 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t967));
            let t972 = t971 * t26;
            let t979 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t972 * t52 - f64x8::splat(3.0) / f64x8::splat(8.0) * t907 + t830 / f64x8::splat(4.0) - t694));
            let t980 = t444 * t176;
            let t987 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), f64x8::splat(6.0) * t270 - f64x8::splat(6.0) * t771)));
            let t991 = ((t62).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t764 * t980 + f64x8::splat(4.0) / f64x8::splat(3.0) * t397 * t449 + f64x8::splat(4.0) / f64x8::splat(3.0) * t63 * t987));
            let t992 = t991 * t26;
            let t1008 = f64x8::splat(1.0) / t69 / t246;
            let t1009 = v_sigma2 * t1008;
            let t1013 = t464 * t192;
            let t1014 = t1013 * t208;
            let t1017 = t186 * t473;
            let t1018 = t1017 * t475;
            let t1021 = t469 * t498;
            let t1024 = (simd::pow(t78, f64x8::splat(97.0)));
            let t1025 = t71 * t1024;
            let t1026 = t475 * t208;
            let t1027 = t1025 * t1026;
            let t1030 = t208 * t498;
            let t1038 = f64x8::splat(1.0) / t68 / t490;
            let t1043 = t490 * t184;
            let t1044 = f64x8::splat(1.0) / t1043;
            let t1048 = t197 * t197;
            let t1049 = t490 * t246;
            let t1051 = f64x8::splat(1.0) / t69 / t1049;
            let t1053 = t203 * t203;
            let t1054 = f64x8::splat(1.0) / t1053;
            let t1056 = t1054 * t28 * t32;
            let t1059 = -f64x8::splat(1232.0) / f64x8::splat(27.0) * t33 * t1009 * t76 + f64x8::splat(184.82618694493908) * t124 * t197 * t1038 * t204 - f64x8::splat(13.924468709572384) * t489 * t1044 * t495 + f64x8::splat(5.362507665863426) * t1048 * t1051 * t1056;
            let t1060 = t193 * t1059;
            let t1063 = f64x8::splat(0.41734945439179794) * t33 * t1009 * t81 - f64x8::splat(0.003580180844611904) * t191 * t1014 + f64x8::splat(0.09666488280452142) * t191 * t1018 + f64x8::splat(0.0009764129576214284) * t191 * t1021 - f64x8::splat(1.1841448143553872) * t191 * t1027 - f64x8::splat(0.036249331051695526) * t191 * t474 * t1030 - f64x8::splat(0.00012205161970267855) * t191 * t1060;
            let t1068 = ((t57).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t992 * t85 - f64x8::splat(3.0) / f64x8::splat(8.0) * t937 - f64x8::splat(9.0) / f64x8::splat(8.0) * t5 * t454 * t212 + t870 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t944 - f64x8::splat(9.0) / f64x8::splat(8.0) * t5 * t180 * t502 - t794 + t881 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t952 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t66 * t1063));
            let tv3rho33 = f64x8::splat(3.0) * t443 + f64x8::splat(3.0) * t507 + t6 * (t979 + t1068);
            acc_v3rho3_3 = tv3rho33;
            let t1076 = t5 * t283 * t236;
            let t1083 = t5 * t292 * t236 / f64x8::splat(12.0);
            let t1085 = t5 * t104 * t544;
            let t1096 = t707 * t232;
            let t1102 = t305 * t540;
            let t1105 = t232 * t311;
            let t1109 = t540 * t136;
            let t1113 = t232 * t334;
            let t1120 = t320 * t132;
            let t1124 = t328 * t331;
            let t1127 = t326 * t126;
            let t1129 = f64x8::splat(1.0) / t36 / t1127;
            let t1133 = f64x8::splat(88.0) / f64x8::splat(9.0) * t33 * t300 * t43 - f64x8::splat(52.84619714701338) * t124 * t1120 * v_sigma0 + f64x8::splat(4.672025685448629) * t1124 * t125 - f64x8::splat(2.0109403746987846) * t325 * t1129 * t750;
            let t1134 = t117 * t1133;
            let t1137 = -f64x8::splat(0.08943202594109956) * t33 * t300 * t48 + f64x8::splat(0.0006509419717476189) * t33 * t306 - f64x8::splat(0.012083110350565177) * t33 * t312 - f64x8::splat(0.00012205161970267855) * t33 * t335 - f64x8::splat(0.0011933936148706347) * t115 * t1096 + f64x8::splat(0.06444325520301428) * t115 * t711 * t524 + f64x8::splat(0.0006509419717476189) * t115 * t1102 - f64x8::splat(1.1841448143553872) * t115 * t719 * t1105 - f64x8::splat(0.024166220701130354) * t115 * t310 * t1109 - f64x8::splat(0.012083110350565177) * t115 * t310 * t1113 - f64x8::splat(0.00012205161970267855) * t115 * t1134;
            let t1142 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t279 * t236 - t1076 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t5 * t98 * t544 + t1083 - t1085 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t1137));
            let tv3rho2sigma0 = t6 * t1142 + f64x8::splat(2.0) * t549;
            acc_v3rho2sigma_0 = tv3rho2sigma0;
            let tv3rho2sigma1 = f64x8::splat(0.0);
            acc_v3rho2sigma_1 = tv3rho2sigma1;
            let t1149 = t5 * t361 * t257;
            let t1153 = t5 * t365 * t257 / f64x8::splat(12.0);
            let t1155 = ((t57).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t357 * t257 - t1149 / f64x8::splat(4.0) + t1153));
            let tv3rho2sigma2 = t6 * t1155 + f64x8::splat(2.0) * t558;
            acc_v3rho2sigma_2 = tv3rho2sigma2;
            let t1161 = t5 * t386 * t236;
            let t1169 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t382 * t236 - t1161 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t169 * t544 - t1076 / f64x8::splat(8.0) + t1083 - t1085 / f64x8::splat(8.0)));
            let tv3rho2sigma3 = t6 * t1169 + t549 + t564;
            acc_v3rho2sigma_3 = tv3rho2sigma3;
            let tv3rho2sigma4 = f64x8::splat(0.0);
            acc_v3rho2sigma_4 = tv3rho2sigma4;
            let t1175 = t5 * t410 * t257;
            let t1182 = t5 * t156 * t597;
            let t1185 = ((t57).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t406 * t257 - t1175 / f64x8::splat(8.0) - t1149 / f64x8::splat(8.0) + t1153 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t152 * t597 - t1182 / f64x8::splat(8.0)));
            let tv3rho2sigma5 = t6 * t1185 + t558 + t602;
            acc_v3rho2sigma_5 = tv3rho2sigma5;
            let t1193 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t437 * t236 - t1161 / f64x8::splat(4.0) + t1083));
            let tv3rho2sigma6 = t6 * t1193 + f64x8::splat(2.0) * t564;
            acc_v3rho2sigma_6 = tv3rho2sigma6;
            let tv3rho2sigma7 = f64x8::splat(0.0);
            acc_v3rho2sigma_7 = tv3rho2sigma7;
            let t1213 = t1013 * t253;
            let t1219 = t469 * t593;
            let t1222 = t253 * t475;
            let t1226 = t593 * t208;
            let t1230 = t253 * t498;
            let t1237 = t484 * t204;
            let t1241 = t492 * t495;
            let t1244 = t490 * t198;
            let t1246 = f64x8::splat(1.0) / t69 / t1244;
            let t1250 = f64x8::splat(88.0) / f64x8::splat(9.0) * t33 * t464 * t76 - f64x8::splat(52.84619714701338) * t124 * t1237 * v_sigma2 + f64x8::splat(4.672025685448629) * t1241 * t197 - f64x8::splat(2.0109403746987846) * t489 * t1246 * t1056;
            let t1251 = t193 * t1250;
            let t1254 = -f64x8::splat(0.08943202594109956) * t33 * t464 * t81 + f64x8::splat(0.0006509419717476189) * t33 * t470 - f64x8::splat(0.012083110350565177) * t33 * t476 - f64x8::splat(0.00012205161970267855) * t33 * t499 - f64x8::splat(0.0011933936148706347) * t191 * t1213 + f64x8::splat(0.06444325520301428) * t191 * t1017 * t577 + f64x8::splat(0.0006509419717476189) * t191 * t1219 - f64x8::splat(1.1841448143553872) * t191 * t1025 * t1222 - f64x8::splat(0.024166220701130354) * t191 * t474 * t1226 - f64x8::splat(0.012083110350565177) * t191 * t474 * t1230 - f64x8::splat(0.00012205161970267855) * t191 * t1251;
            let t1259 = ((t57).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t454 * t257 - t1175 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t5 * t180 * t597 + t1153 - t1182 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t66 * t1254));
            let tv3rho2sigma8 = t6 * t1259 + f64x8::splat(2.0) * t602;
            acc_v3rho2sigma_8 = tv3rho2sigma8;
            let t1266 = t5 * t104 * t621 / f64x8::splat(8.0);
            let t1269 = t33 * t38;
            let t1270 = t309 * t232;
            let t1271 = t1270 * t136;
            let t1276 = t711 * t606;
            let t1279 = t606 * t136;
            let t1283 = t232 * t540;
            let t1287 = t305 * t617;
            let t1290 = t617 * t136;
            let t1296 = t536 * t331;
            let t1300 = f64x8::splat(1.0) / t36 / t737;
            let t1304 = f64x8::splat(9.756221011756315) * t124 * t531 - f64x8::splat(1.3741252016025378) * t1296 * v_sigma0 + f64x8::splat(0.7541026405120442) * t125 * t1300 * t750;
            let t1305 = t117 * t1304;
            let t1308 = f64x8::splat(0.0006509419717476189) * t33 * t521 - f64x8::splat(0.024166220701130354) * t1269 * t1271 - f64x8::splat(0.0002441032394053571) * t33 * t541 + f64x8::splat(0.03222162760150714) * t115 * t1276 - f64x8::splat(1.1841448143553872) * t115 * t719 * t1279 - f64x8::splat(0.024166220701130354) * t115 * t310 * t1283 + f64x8::splat(0.00032547098587380947) * t115 * t1287 - f64x8::splat(0.012083110350565177) * t115 * t310 * t1290 - f64x8::splat(0.00012205161970267855) * t115 * t1305;
            let t1313 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t98 * t621 - t1266 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t1308));
            let tv3rhosigma20 = t6 * t1313 + t625;
            acc_v3rhosigma2_0 = tv3rhosigma20;
            let tv3rhosigma21 = f64x8::splat(0.0);
            acc_v3rhosigma2_1 = tv3rhosigma21;
            let tv3rhosigma22 = f64x8::splat(0.0);
            acc_v3rhosigma2_2 = tv3rhosigma22;
            let tv3rhosigma23 = f64x8::splat(0.0);
            acc_v3rhosigma2_3 = tv3rhosigma23;
            let tv3rhosigma24 = f64x8::splat(0.0);
            acc_v3rhosigma2_4 = tv3rhosigma24;
            let t1320 = t5 * t156 * t643 / f64x8::splat(8.0);
            let t1322 = ((t57).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t152 * t643 - t1320));
            let tv3rhosigma25 = t6 * t1322 + t647;
            acc_v3rhosigma2_5 = tv3rhosigma25;
            let t1328 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t169 * t621 - t1266));
            let tv3rhosigma26 = t6 * t1328 + t625;
            acc_v3rhosigma2_6 = tv3rhosigma26;
            let tv3rhosigma27 = f64x8::splat(0.0);
            acc_v3rhosigma2_7 = tv3rhosigma27;
            let tv3rhosigma28 = f64x8::splat(0.0);
            acc_v3rhosigma2_8 = tv3rhosigma28;
            let tv3rhosigma29 = f64x8::splat(0.0);
            acc_v3rhosigma2_9 = tv3rhosigma29;
            let tv3rhosigma210 = f64x8::splat(0.0);
            acc_v3rhosigma2_10 = tv3rhosigma210;
            let t1335 = t33 * t71;
            let t1336 = t473 * t253;
            let t1337 = t1336 * t208;
            let t1342 = t1017 * t628;
            let t1345 = t628 * t208;
            let t1349 = t253 * t593;
            let t1353 = t469 * t639;
            let t1356 = t639 * t208;
            let t1362 = t589 * t495;
            let t1366 = f64x8::splat(1.0) / t69 / t1043;
            let t1370 = f64x8::splat(9.756221011756315) * t124 * t584 - f64x8::splat(1.3741252016025378) * t1362 * v_sigma2 + f64x8::splat(0.7541026405120442) * t197 * t1366 * t1056;
            let t1371 = t193 * t1370;
            let t1374 = f64x8::splat(0.0006509419717476189) * t33 * t574 - f64x8::splat(0.024166220701130354) * t1335 * t1337 - f64x8::splat(0.0002441032394053571) * t33 * t594 + f64x8::splat(0.03222162760150714) * t191 * t1342 - f64x8::splat(1.1841448143553872) * t191 * t1025 * t1345 - f64x8::splat(0.024166220701130354) * t191 * t474 * t1349 + f64x8::splat(0.00032547098587380947) * t191 * t1353 - f64x8::splat(0.012083110350565177) * t191 * t474 * t1356 - f64x8::splat(0.00012205161970267855) * t191 * t1371;
            let t1379 = ((t57).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t180 * t643 - t1320 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t66 * t1374));
            let tv3rhosigma211 = t6 * t1379 + t647;
            acc_v3rhosigma2_11 = tv3rhosigma211;
            let t1385 = t606 * t232;
            let t1386 = t719 * t1385;
            let t1389 = t232 * t617;
            let t1396 = f64x8::splat(1.0) / t36 / t327;
            let t1400 = f64x8::splat(0.30917817036057105) * t613 * t331 - f64x8::splat(0.2827884901920166) * v_sigma0 * t1396 * t750;
            let t1401 = t117 * t1400;
            let t1404 = -f64x8::splat(0.036249331051695526) * t33 * t607 - f64x8::splat(0.0003661548591080356) * t33 * t618 - f64x8::splat(1.1841448143553872) * t115 * t1386 - f64x8::splat(0.036249331051695526) * t115 * t310 * t1389 - f64x8::splat(0.00012205161970267855) * t115 * t1401;
            let t1408 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t1404));
            let tv3sigma30 = t6 * t1408;
            acc_v3sigma3_0 = tv3sigma30;
            let tv3sigma31 = f64x8::splat(0.0);
            acc_v3sigma3_1 = tv3sigma31;
            let tv3sigma32 = f64x8::splat(0.0);
            acc_v3sigma3_2 = tv3sigma32;
            let tv3sigma33 = f64x8::splat(0.0);
            acc_v3sigma3_3 = tv3sigma33;
            let tv3sigma34 = f64x8::splat(0.0);
            acc_v3sigma3_4 = tv3sigma34;
            let tv3sigma35 = f64x8::splat(0.0);
            acc_v3sigma3_5 = tv3sigma35;
            let tv3sigma36 = f64x8::splat(0.0);
            acc_v3sigma3_6 = tv3sigma36;
            let tv3sigma37 = f64x8::splat(0.0);
            acc_v3sigma3_7 = tv3sigma37;
            let tv3sigma38 = f64x8::splat(0.0);
            acc_v3sigma3_8 = tv3sigma38;
            let t1413 = t628 * t253;
            let t1414 = t1025 * t1413;
            let t1417 = t253 * t639;
            let t1424 = f64x8::splat(1.0) / t69 / t491;
            let t1428 = f64x8::splat(0.30917817036057105) * t635 * t495 - f64x8::splat(0.2827884901920166) * v_sigma2 * t1424 * t1056;
            let t1429 = t193 * t1428;
            let t1432 = -f64x8::splat(0.036249331051695526) * t33 * t629 - f64x8::splat(0.0003661548591080356) * t33 * t640 - f64x8::splat(1.1841448143553872) * t191 * t1414 - f64x8::splat(0.036249331051695526) * t191 * t474 * t1417 - f64x8::splat(0.00012205161970267855) * t191 * t1429;
            let t1436 = ((t57).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t66 * t1432));
            let tv3sigma39 = t6 * t1436;
            acc_v3sigma3_9 = tv3sigma39;
        }
        store_add(zk, ip, m, acc_zk);
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(vsigma, ip, m, 3, 0, acc_vsigma_0);
        store_strided(vsigma, ip, m, 3, 1, acc_vsigma_1);
        store_strided(vsigma, ip, m, 3, 2, acc_vsigma_2);
        store_strided(v2rho2, ip, m, 3, 0, acc_v2rho2_0);
        store_strided(v2rho2, ip, m, 3, 1, acc_v2rho2_1);
        store_strided(v2rho2, ip, m, 3, 2, acc_v2rho2_2);
        store_strided(v2rhosigma, ip, m, 6, 0, acc_v2rhosigma_0);
        store_strided(v2rhosigma, ip, m, 6, 1, acc_v2rhosigma_1);
        store_strided(v2rhosigma, ip, m, 6, 2, acc_v2rhosigma_2);
        store_strided(v2rhosigma, ip, m, 6, 3, acc_v2rhosigma_3);
        store_strided(v2rhosigma, ip, m, 6, 4, acc_v2rhosigma_4);
        store_strided(v2rhosigma, ip, m, 6, 5, acc_v2rhosigma_5);
        store_strided(v2sigma2, ip, m, 6, 0, acc_v2sigma2_0);
        store_strided(v2sigma2, ip, m, 6, 1, acc_v2sigma2_1);
        store_strided(v2sigma2, ip, m, 6, 2, acc_v2sigma2_2);
        store_strided(v2sigma2, ip, m, 6, 3, acc_v2sigma2_3);
        store_strided(v2sigma2, ip, m, 6, 4, acc_v2sigma2_4);
        store_strided(v2sigma2, ip, m, 6, 5, acc_v2sigma2_5);
        store_strided(v3rho3, ip, m, 4, 0, acc_v3rho3_0);
        store_strided(v3rho3, ip, m, 4, 1, acc_v3rho3_1);
        store_strided(v3rho3, ip, m, 4, 2, acc_v3rho3_2);
        store_strided(v3rho3, ip, m, 4, 3, acc_v3rho3_3);
        store_strided(v3rho2sigma, ip, m, 9, 0, acc_v3rho2sigma_0);
        store_strided(v3rho2sigma, ip, m, 9, 1, acc_v3rho2sigma_1);
        store_strided(v3rho2sigma, ip, m, 9, 2, acc_v3rho2sigma_2);
        store_strided(v3rho2sigma, ip, m, 9, 3, acc_v3rho2sigma_3);
        store_strided(v3rho2sigma, ip, m, 9, 4, acc_v3rho2sigma_4);
        store_strided(v3rho2sigma, ip, m, 9, 5, acc_v3rho2sigma_5);
        store_strided(v3rho2sigma, ip, m, 9, 6, acc_v3rho2sigma_6);
        store_strided(v3rho2sigma, ip, m, 9, 7, acc_v3rho2sigma_7);
        store_strided(v3rho2sigma, ip, m, 9, 8, acc_v3rho2sigma_8);
        store_strided(v3rhosigma2, ip, m, 12, 0, acc_v3rhosigma2_0);
        store_strided(v3rhosigma2, ip, m, 12, 1, acc_v3rhosigma2_1);
        store_strided(v3rhosigma2, ip, m, 12, 2, acc_v3rhosigma2_2);
        store_strided(v3rhosigma2, ip, m, 12, 3, acc_v3rhosigma2_3);
        store_strided(v3rhosigma2, ip, m, 12, 4, acc_v3rhosigma2_4);
        store_strided(v3rhosigma2, ip, m, 12, 5, acc_v3rhosigma2_5);
        store_strided(v3rhosigma2, ip, m, 12, 6, acc_v3rhosigma2_6);
        store_strided(v3rhosigma2, ip, m, 12, 7, acc_v3rhosigma2_7);
        store_strided(v3rhosigma2, ip, m, 12, 8, acc_v3rhosigma2_8);
        store_strided(v3rhosigma2, ip, m, 12, 9, acc_v3rhosigma2_9);
        store_strided(v3rhosigma2, ip, m, 12, 10, acc_v3rhosigma2_10);
        store_strided(v3rhosigma2, ip, m, 12, 11, acc_v3rhosigma2_11);
        store_strided(v3sigma3, ip, m, 10, 0, acc_v3sigma3_0);
        store_strided(v3sigma3, ip, m, 10, 1, acc_v3sigma3_1);
        store_strided(v3sigma3, ip, m, 10, 2, acc_v3sigma3_2);
        store_strided(v3sigma3, ip, m, 10, 3, acc_v3sigma3_3);
        store_strided(v3sigma3, ip, m, 10, 4, acc_v3sigma3_4);
        store_strided(v3sigma3, ip, m, 10, 5, acc_v3sigma3_5);
        store_strided(v3sigma3, ip, m, 10, 6, acc_v3sigma3_6);
        store_strided(v3sigma3, ip, m, 10, 7, acc_v3sigma3_7);
        store_strided(v3sigma3, ip, m, 10, 8, acc_v3sigma3_8);
        store_strided(v3sigma3, ip, m, 10, 9, acc_v3sigma3_9);
        ip += 8;
    }
}
