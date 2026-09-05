//! GGA_X_CAP lxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_cap.c`
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

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_cap_lxc_unpol(
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
    v4rho4: &mut [f64],
    v4rho3sigma: &mut [f64],
    v4rho2sigma2: &mut [f64],
    v4rhosigma3: &mut [f64],
    v4sigma4: &mut [f64],
    param_alphaoAx: f64,
    param_c: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_alphaoAx = f64x8::splat(param_alphaoAx);
    let param_c = f64x8::splat(param_c);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_vsigma = V_ZERO;
        let mut acc_v2rho2 = V_ZERO;
        let mut acc_v2rhosigma = V_ZERO;
        let mut acc_v2sigma2 = V_ZERO;
        let mut acc_v3rho3 = V_ZERO;
        let mut acc_v3rho2sigma = V_ZERO;
        let mut acc_v3rhosigma2 = V_ZERO;
        let mut acc_v3sigma3 = V_ZERO;
        let mut acc_v4rho4 = V_ZERO;
        let mut acc_v4rho3sigma = V_ZERO;
        let mut acc_v4rho2sigma2 = V_ZERO;
        let mut acc_v4rhosigma3 = V_ZERO;
        let mut acc_v4sigma4 = V_ZERO;
        {
            let t2 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t3 = f64x8::splat(M_CBRT3);
            let t4 = f64x8::splat(M_CBRTPI);
            let t6 = t3 / t4;
            let t7 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t8 = zeta_threshold - f64x8::splat(1.0);
            let t10 = ((t7).select(t8, (t7).select(-t8, f64x8::splat(0.0))));
            let t11 = f64x8::splat(1.0) + t10;
            let t13 = (simd::cbrt(zeta_threshold));
            let t15 = (simd::cbrt(t11));
            let t17 = (((t11).simd_le(zeta_threshold)).select(t13 * zeta_threshold, t15 * t11));
            let t18 = (simd::cbrt(v_rho));
            let t19 = t17 * t18;
            let t20 = f64x8::splat(M_CBRT6);
            let t21 = t20 * t20;
            let t22 = param_alphaoAx * t21;
            let t23 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t24 = (simd::cbrt(t23));
            let t25 = f64x8::splat(1.0) / t24;
            let t26 = ((v_sigma).sqrt());
            let t28 = t22 * t25 * t26;
            let t29 = f64x8::splat(M_CBRT2);
            let t31 = f64x8::splat(1.0) / t18 / v_rho;
            let t33 = t21 * t25;
            let t38 = f64x8::splat(1.0) + t33 * t26 * t29 * t31 / f64x8::splat(12.0);
            let t39 = (simd::ln(t38));
            let t41 = param_c * t39 + f64x8::splat(1.0);
            let t42 = f64x8::splat(1.0) / t41;
            let t43 = t39 * t42;
            let t44 = t29 * t31 * t43;
            let t47 = f64x8::splat(1.0) - t28 * t44 / f64x8::splat(12.0);
            let t51 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t47));
            let tzk0 = f64x8::splat(2.0) * t51;
            acc_zk = tzk0;
            let t52 = t18 * t18;
            let t54 = t17 / t52;
            let t58 = v_rho * v_rho;
            let t62 = t29 / t18 / t58 * t43;
            let t65 = param_alphaoAx * t20;
            let t66 = t24 * t24;
            let t67 = f64x8::splat(1.0) / t66;
            let t68 = t67 * v_sigma;
            let t69 = t65 * t68;
            let t70 = t29 * t29;
            let t71 = t58 * v_rho;
            let t73 = f64x8::splat(1.0) / t52 / t71;
            let t75 = f64x8::splat(1.0) / t38;
            let t76 = t75 * t42;
            let t77 = t70 * t73 * t76;
            let t81 = t65 * t68 * t70;
            let t83 = t41 * t41;
            let t84 = f64x8::splat(1.0) / t83;
            let t85 = t84 * param_c;
            let t86 = t85 * t75;
            let t87 = t73 * t39 * t86;
            let t90 = t28 * t62 / f64x8::splat(9.0) + t69 * t77 / f64x8::splat(18.0) - t81 * t87 / f64x8::splat(18.0);
            let t95 = ((t2).select(f64x8::splat(0.0), -t6 * t54 * t47 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t90));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t95 + f64x8::splat(2.0) * t51;
            acc_vrho = tvrho0;
            let t98 = f64x8::splat(1.0) / t26;
            let t100 = t22 * t25 * t98;
            let t103 = t65 * t67;
            let t105 = f64x8::splat(1.0) / t52 / t58;
            let t107 = t70 * t105 * t76;
            let t110 = t67 * t70;
            let t111 = t65 * t110;
            let t113 = t105 * t39 * t86;
            let t116 = -t100 * t44 / f64x8::splat(24.0) - t103 * t107 / f64x8::splat(48.0) + t111 * t113 / f64x8::splat(48.0);
            let t120 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t116));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t120;
            acc_vsigma = tvsigma0;
            let t125 = t17 / t52 / v_rho;
            let t135 = t29 / t18 / t71 * t43;
            let t138 = t58 * t58;
            let t140 = f64x8::splat(1.0) / t52 / t138;
            let t142 = t70 * t140 * t76;
            let t146 = t140 * t39 * t86;
            let t150 = param_alphaoAx / t23;
            let t151 = t26 * v_sigma;
            let t152 = t150 * t151;
            let t153 = t138 * t58;
            let t154 = f64x8::splat(1.0) / t153;
            let t155 = t38 * t38;
            let t156 = f64x8::splat(1.0) / t155;
            let t157 = t154 * t156;
            let t158 = t157 * t42;
            let t161 = t157 * t85;
            let t165 = t150 * t151 * t154;
            let t167 = f64x8::splat(1.0) / t83 / t41;
            let t169 = param_c * param_c;
            let t171 = t39 * t167 * t169 * t156;
            let t176 = t39 * t84 * param_c * t156;
            let t179 = -f64x8::splat(7.0) / f64x8::splat(27.0) * t28 * t135 - f64x8::splat(5.0) / f64x8::splat(18.0) * t69 * t142 + f64x8::splat(5.0) / f64x8::splat(18.0) * t81 * t146 + f64x8::splat(2.0) / f64x8::splat(27.0) * t152 * t158 + f64x8::splat(4.0) / f64x8::splat(27.0) * t152 * t161 - f64x8::splat(4.0) / f64x8::splat(27.0) * t165 * t171 - f64x8::splat(2.0) / f64x8::splat(27.0) * t165 * t176;
            let t184 = ((t2).select(f64x8::splat(0.0), t6 * t125 * t47 / f64x8::splat(12.0) - t6 * t54 * t90 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t179));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t184 + f64x8::splat(4.0) * t95;
            acc_v2rho2 = tv2rho20;
            let t196 = t138 * v_rho;
            let t197 = f64x8::splat(1.0) / t196;
            let t198 = t150 * t197;
            let t199 = t156 * t42;
            let t200 = t199 * t26;
            let t203 = t156 * t84;
            let t205 = t203 * param_c * t26;
            let t209 = t150 * t197 * t39;
            let t210 = t167 * t169;
            let t212 = t210 * t156 * t26;
            let t217 = t100 * t62 / f64x8::splat(18.0) + t103 * t77 / f64x8::splat(12.0) - t111 * t87 / f64x8::splat(12.0) - t198 * t200 / f64x8::splat(36.0) - t198 * t205 / f64x8::splat(18.0) + t209 * t212 / f64x8::splat(18.0) + t209 * t205 / f64x8::splat(36.0);
            let t222 = ((t2).select(f64x8::splat(0.0), -t6 * t54 * t116 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t217));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t222 + f64x8::splat(2.0) * t120;
            acc_v2rhosigma = tv2rhosigma0;
            let t225 = f64x8::splat(1.0) / t151;
            let t227 = t22 * t25 * t225;
            let t230 = f64x8::splat(1.0) / v_sigma;
            let t231 = t67 * t230;
            let t232 = t65 * t231;
            let t236 = t65 * t231 * t70;
            let t239 = f64x8::splat(1.0) / t138;
            let t240 = t150 * t239;
            let t245 = t203 * param_c * t98;
            let t249 = t150 * t239 * t39;
            let t256 = t227 * t44 / f64x8::splat(48.0) - t232 * t107 / f64x8::splat(96.0) + t236 * t113 / f64x8::splat(96.0) + t240 * t199 * t98 / f64x8::splat(96.0) + t240 * t245 / f64x8::splat(48.0) - t249 * t210 * t156 * t98 / f64x8::splat(48.0) - t249 * t245 / f64x8::splat(96.0);
            let t260 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t256));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t260;
            acc_v2sigma2 = tv2sigma20;
            let t263 = t17 * t105;
            let t276 = t29 / t18 / t138 * t43;
            let t280 = f64x8::splat(1.0) / t52 / t196;
            let t282 = t70 * t280 * t76;
            let t286 = t280 * t39 * t86;
            let t289 = t138 * t71;
            let t290 = f64x8::splat(1.0) / t289;
            let t291 = t290 * t156;
            let t299 = t150 * t151 * t290;
            let t304 = v_sigma * v_sigma;
            let t305 = t138 * t138;
            let t307 = f64x8::splat(1.0) / t18 / t305;
            let t308 = t304 * t307;
            let t311 = f64x8::splat(1.0) / t155 / t38;
            let t313 = t33 * t29;
            let t314 = t311 * t42 * t313;
            let t318 = t150 * t308 * t311;
            let t319 = t85 * t313;
            let t322 = t210 * t313;
            let t326 = t150 * t308 * t39;
            let t327 = t83 * t83;
            let t328 = f64x8::splat(1.0) / t327;
            let t329 = t169 * param_c;
            let t330 = t328 * t329;
            let t332 = t330 * t311 * t313;
            let t336 = t210 * t311 * t313;
            let t340 = t85 * t311 * t313;
            let t343 = f64x8::splat(70.0) / f64x8::splat(81.0) * t28 * t276 + f64x8::splat(119.0) / f64x8::splat(81.0) * t69 * t282 - f64x8::splat(119.0) / f64x8::splat(81.0) * t81 * t286 - f64x8::splat(22.0) / f64x8::splat(27.0) * t152 * t291 * t42 - f64x8::splat(44.0) / f64x8::splat(27.0) * t152 * t291 * t85 + f64x8::splat(44.0) / f64x8::splat(27.0) * t299 * t171 + f64x8::splat(22.0) / f64x8::splat(27.0) * t299 * t176 + f64x8::splat(4.0) / f64x8::splat(243.0) * t150 * t308 * t314 + f64x8::splat(4.0) / f64x8::splat(81.0) * t318 * t319 + f64x8::splat(4.0) / f64x8::splat(81.0) * t318 * t322 - f64x8::splat(4.0) / f64x8::splat(81.0) * t326 * t332 - f64x8::splat(4.0) / f64x8::splat(81.0) * t326 * t336 - f64x8::splat(4.0) / f64x8::splat(243.0) * t326 * t340;
            let t348 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(5.0) / f64x8::splat(36.0) * t6 * t263 * t47 + t6 * t125 * t90 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t54 * t179 - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t343));
            let tv3rho30 = f64x8::splat(2.0) * v_rho * t348 + f64x8::splat(6.0) * t184;
            acc_v3rho3 = tv3rho30;
            let t364 = t150 * t154;
            let t370 = t150 * t154 * t39;
            let t376 = f64x8::splat(1.0) / t18 / t289;
            let t377 = t376 * t311;
            let t378 = t150 * t377;
            let t380 = t42 * v_sigma * t313;
            let t386 = v_sigma * param_c * t313;
            let t392 = t169 * v_sigma * t313;
            let t395 = t376 * t39;
            let t397 = t150 * t395 * t328;
            let t398 = t329 * t311;
            let t400 = t398 * v_sigma * t313;
            let t404 = t150 * t395 * t167;
            let t405 = t169 * t311;
            let t407 = t405 * v_sigma * t313;
            let t411 = t150 * t395 * t84;
            let t412 = param_c * t311;
            let t414 = t412 * v_sigma * t313;
            let t417 = -f64x8::splat(7.0) / f64x8::splat(54.0) * t100 * t135 - f64x8::splat(37.0) / f64x8::splat(108.0) * t103 * t142 + f64x8::splat(37.0) / f64x8::splat(108.0) * t111 * t146 + t364 * t200 / f64x8::splat(4.0) + t364 * t205 / f64x8::splat(2.0) - t370 * t212 / f64x8::splat(2.0) - t370 * t205 / f64x8::splat(4.0) - t378 * t380 / f64x8::splat(162.0) - t150 * t377 * t84 * t386 / f64x8::splat(54.0) - t150 * t377 * t167 * t392 / f64x8::splat(54.0) + t397 * t400 / f64x8::splat(54.0) + t404 * t407 / f64x8::splat(54.0) + t411 * t414 / f64x8::splat(162.0);
            let t422 = ((t2).select(f64x8::splat(0.0), t6 * t125 * t116 / f64x8::splat(12.0) - t6 * t54 * t217 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t417));
            let tv3rho2sigma0 = f64x8::splat(2.0) * v_rho * t422 + f64x8::splat(4.0) * t222;
            acc_v3rho2sigma = tv3rho2sigma0;
            let t434 = t150 * t98;
            let t435 = t197 * t156;
            let t436 = t435 * t42;
            let t439 = t435 * t85;
            let t443 = t150 * t98 * t197;
            let t449 = f64x8::splat(1.0) / t18 / t153;
            let t450 = t449 * t311;
            let t451 = t150 * t450;
            let t454 = t42 * t21 * t25 * t29;
            let t461 = t449 * t39;
            let t463 = t150 * t461 * t328;
            let t464 = t398 * t313;
            let t468 = t150 * t461 * t167;
            let t469 = t405 * t313;
            let t473 = t150 * t461 * t84;
            let t474 = t412 * t313;
            let t477 = -t227 * t62 / f64x8::splat(36.0) + t232 * t77 / f64x8::splat(72.0) - t236 * t87 / f64x8::splat(72.0) - t434 * t436 / f64x8::splat(18.0) - t434 * t439 / f64x8::splat(9.0) + t443 * t171 / f64x8::splat(9.0) + t443 * t176 / f64x8::splat(18.0) + t451 * t454 / f64x8::splat(432.0) + t451 * t319 / f64x8::splat(144.0) + t451 * t322 / f64x8::splat(144.0) - t463 * t464 / f64x8::splat(144.0) - t468 * t469 / f64x8::splat(144.0) - t473 * t474 / f64x8::splat(432.0);
            let t482 = ((t2).select(f64x8::splat(0.0), -t6 * t54 * t256 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t477));
            let tv3rhosigma20 = f64x8::splat(2.0) * v_rho * t482 + f64x8::splat(2.0) * t260;
            acc_v3rhosigma2 = tv3rhosigma20;
            let t485 = t26 * t304;
            let t486 = f64x8::splat(1.0) / t485;
            let t488 = t22 * t25 * t486;
            let t491 = f64x8::splat(1.0) / t304;
            let t492 = t67 * t491;
            let t493 = t65 * t492;
            let t497 = t65 * t492 * t70;
            let t501 = f64x8::splat(1.0) / t18 / t196;
            let t502 = t501 * t311;
            let t503 = t150 * t502;
            let t505 = t42 * t230 * t313;
            let t509 = t150 * t502 * t84;
            let t511 = t230 * param_c * t313;
            let t515 = t150 * t502 * t167;
            let t517 = t169 * t230 * t313;
            let t520 = t501 * t39;
            let t522 = t150 * t520 * t328;
            let t524 = t398 * t230 * t313;
            let t528 = t150 * t520 * t167;
            let t530 = t405 * t230 * t313;
            let t534 = t150 * t520 * t84;
            let t536 = t412 * t230 * t313;
            let t539 = -t488 * t44 / f64x8::splat(32.0) + t493 * t107 / f64x8::splat(64.0) - t497 * t113 / f64x8::splat(64.0) - t503 * t505 / f64x8::splat(1152.0) - t509 * t511 / f64x8::splat(384.0) - t515 * t517 / f64x8::splat(384.0) + t522 * t524 / f64x8::splat(384.0) + t528 * t530 / f64x8::splat(384.0) + t534 * t536 / f64x8::splat(1152.0);
            let t543 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t539));
            let tv3sigma30 = f64x8::splat(2.0) * v_rho * t543;
            acc_v3sigma3 = tv3sigma30;
            let t559 = t305 * v_rho;
            let t562 = t304 / t18 / t559;
            let t569 = t485 / t52 / t305 / t58;
            let t571 = t155 * t155;
            let t572 = f64x8::splat(1.0) / t571;
            let t574 = t20 * t67;
            let t575 = t574 * t70;
            let t583 = f64x8::splat(1.0) / t305;
            let t584 = t583 * t156;
            let t589 = f64x8::splat(1.0) / t52 / t153;
            let t595 = t150 * t151 * t583;
            let t601 = t150 * t569 * t572;
            let t603 = t110 * param_c;
            let t614 = t589 * t39;
            let t619 = t150 * t562 * t311;
            let t628 = t150 * t569 * t39;
            let t630 = f64x8::splat(1.0) / t327 / t41;
            let t631 = t169 * t169;
            let t650 = t150 * t562 * t39;
            let t657 = f64x8::splat(721.0) / f64x8::splat(81.0) * t81 * t614 * t86 - f64x8::splat(232.0) / f64x8::splat(243.0) * t619 * t319 - f64x8::splat(232.0) / f64x8::splat(243.0) * t619 * t322 + f64x8::splat(3724.0) / f64x8::splat(243.0) * t152 * t584 * t85 - f64x8::splat(32.0) / f64x8::splat(243.0) * t628 * t630 * t631 * t572 * t575 - f64x8::splat(16.0) / f64x8::splat(81.0) * t628 * t330 * t572 * t575 - f64x8::splat(88.0) / f64x8::splat(729.0) * t628 * t210 * t572 * t575 - f64x8::splat(8.0) / f64x8::splat(243.0) * t628 * t85 * t572 * t575 + f64x8::splat(232.0) / f64x8::splat(243.0) * t650 * t332 + f64x8::splat(232.0) / f64x8::splat(243.0) * t650 * t336 + f64x8::splat(232.0) / f64x8::splat(729.0) * t650 * t340;
            let t663 = ((t2).select(f64x8::splat(0.0), f64x8::splat(10.0) / f64x8::splat(27.0) * t6 * t17 * t73 * t47 - f64x8::splat(5.0) / f64x8::splat(9.0) * t6 * t263 * t90 + t6 * t125 * t179 / f64x8::splat(2.0) - t6 * t54 * t343 / f64x8::splat(2.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * (-f64x8::splat(232.0) / f64x8::splat(729.0) * t150 * t562 * t314 + f64x8::splat(8.0) / f64x8::splat(243.0) * t150 * t569 * t572 * t42 * t575 - f64x8::splat(910.0) / f64x8::splat(243.0) * t28 * t29 * t501 * t43 + f64x8::splat(1862.0) / f64x8::splat(243.0) * t152 * t584 * t42 - f64x8::splat(721.0) / f64x8::splat(81.0) * t69 * t70 * t589 * t76 - f64x8::splat(3724.0) / f64x8::splat(243.0) * t595 * t171 - f64x8::splat(1862.0) / f64x8::splat(243.0) * t595 * t176 + f64x8::splat(88.0) / f64x8::splat(729.0) * t601 * t84 * t20 * t603 + f64x8::splat(16.0) / f64x8::splat(81.0) * t601 * t210 * t575 + f64x8::splat(32.0) / f64x8::splat(243.0) * t601 * t330 * t575 + t657)));
            let tv4rho40 = f64x8::splat(2.0) * v_rho * t663 + f64x8::splat(8.0) * t348;
            acc_v4rho4 = tv4rho40;
            let t677 = f64x8::splat(1.0) / t52 / t559;
            let t678 = t677 * t572;
            let t688 = t307 * t311;
            let t692 = t150 * t290;
            let t696 = t150 * t290 * t39;
            let t732 = t677 * t39;
            let t735 = param_c * t572;
            let t742 = t631 * t572;
            let t749 = t329 * t572;
            let t756 = t169 * t572;
            let t761 = t307 * t39;
            let t774 = f64x8::splat(49.0) / f64x8::splat(162.0) * t150 * t688 * t84 * t386 + f64x8::splat(49.0) / f64x8::splat(162.0) * t150 * t688 * t167 * t392 + f64x8::splat(91.0) / f64x8::splat(54.0) * t103 * t282 - f64x8::splat(317.0) / f64x8::splat(81.0) * t692 * t205 + t150 * t732 * t84 * t735 * t151 * t575 / f64x8::splat(81.0) + f64x8::splat(4.0) / f64x8::splat(81.0) * t150 * t732 * t630 * t742 * t151 * t575 + f64x8::splat(2.0) / f64x8::splat(27.0) * t150 * t732 * t328 * t749 * t151 * t575 + f64x8::splat(11.0) / f64x8::splat(243.0) * t150 * t732 * t167 * t756 * t151 * t575 - f64x8::splat(49.0) / f64x8::splat(162.0) * t150 * t761 * t328 * t400 - f64x8::splat(49.0) / f64x8::splat(162.0) * t150 * t761 * t167 * t407 - f64x8::splat(49.0) / f64x8::splat(486.0) * t150 * t761 * t84 * t414;
            let t780 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(5.0) / f64x8::splat(36.0) * t6 * t263 * t116 + t6 * t125 * t217 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t54 * t417 - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * (-t150 * t678 * t42 * t151 * t575 / f64x8::splat(81.0) + f64x8::splat(35.0) / f64x8::splat(81.0) * t100 * t276 - f64x8::splat(91.0) / f64x8::splat(54.0) * t111 * t286 + f64x8::splat(49.0) / f64x8::splat(486.0) * t150 * t688 * t380 - f64x8::splat(317.0) / f64x8::splat(162.0) * t692 * t200 + f64x8::splat(317.0) / f64x8::splat(81.0) * t696 * t212 + f64x8::splat(317.0) / f64x8::splat(162.0) * t696 * t205 - f64x8::splat(11.0) / f64x8::splat(243.0) * t150 * t678 * t84 * t151 * t20 * t603 - f64x8::splat(2.0) / f64x8::splat(27.0) * t150 * t678 * t167 * t151 * t169 * t575 - f64x8::splat(4.0) / f64x8::splat(81.0) * t150 * t678 * t328 * t329 * t151 * t575 + t774)));
            let tv4rho3sigma0 = f64x8::splat(2.0) * v_rho * t780 + f64x8::splat(6.0) * t422;
            acc_v4rho3sigma = tv4rho3sigma0;
            let t791 = f64x8::splat(1.0) / t52 / t305;
            let t792 = t791 * t572;
            let t795 = t110 * t26;
            let t810 = t150 * t98 * t154;
            let t847 = t791 * t39;
            let t872 = t150 * t792 * t328 * t329 * t20 * t795 / f64x8::splat(54.0) + f64x8::splat(7.0) / f64x8::splat(216.0) * t236 * t146 + f64x8::splat(35.0) / f64x8::splat(432.0) * t397 * t464 + f64x8::splat(35.0) / f64x8::splat(432.0) * t404 * t469 + f64x8::splat(35.0) / f64x8::splat(1296.0) * t411 * t474 + f64x8::splat(11.0) / f64x8::splat(648.0) * t150 * t792 * t84 * t574 * t70 * param_c * t26 + f64x8::splat(16.0) / f64x8::splat(27.0) * t434 * t161 - f64x8::splat(11.0) / f64x8::splat(648.0) * t150 * t847 * t167 * t756 * t20 * t795 - t150 * t847 * t84 * t735 * t20 * t795 / f64x8::splat(216.0) - t150 * t847 * t630 * t742 * t20 * t795 / f64x8::splat(54.0) - t150 * t847 * t328 * t749 * t20 * t795 / f64x8::splat(36.0);
            let t878 = ((t2).select(f64x8::splat(0.0), t6 * t125 * t256 / f64x8::splat(12.0) - t6 * t54 * t477 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * (t150 * t792 * t42 * t20 * t795 / f64x8::splat(216.0) + f64x8::splat(7.0) / f64x8::splat(108.0) * t227 * t135 - f64x8::splat(35.0) / f64x8::splat(432.0) * t378 * t319 - f64x8::splat(35.0) / f64x8::splat(432.0) * t378 * t322 + f64x8::splat(8.0) / f64x8::splat(27.0) * t434 * t158 - f64x8::splat(7.0) / f64x8::splat(216.0) * t232 * t142 - f64x8::splat(16.0) / f64x8::splat(27.0) * t810 * t171 - f64x8::splat(8.0) / f64x8::splat(27.0) * t810 * t176 - f64x8::splat(35.0) / f64x8::splat(1296.0) * t378 * t454 + t150 * t792 * t167 * t169 * t20 * t795 / f64x8::splat(36.0) + t872)));
            let tv4rho2sigma20 = f64x8::splat(2.0) * v_rho * t878 + f64x8::splat(4.0) * t482;
            acc_v4rho2sigma2 = tv4rho2sigma20;
            let t885 = t150 * t225 * t197;
            let t892 = t150 * t225;
            let t898 = f64x8::splat(1.0) / t52 / t289;
            let t899 = t898 * t572;
            let t936 = t898 * t39;
            let t969 = t497 * t87 / f64x8::splat(48.0) + t451 * t505 / f64x8::splat(216.0) - f64x8::splat(11.0) / f64x8::splat(1728.0) * t150 * t899 * t84 * t98 * t20 * t603 - t150 * t899 * t167 * t98 * t169 * t575 / f64x8::splat(96.0) + t892 * t439 / f64x8::splat(24.0) + f64x8::splat(11.0) / f64x8::splat(1728.0) * t150 * t936 * t167 * t756 * t98 * t575 + t150 * t936 * t84 * t735 * t98 * t575 / f64x8::splat(576.0) + t150 * t450 * t167 * t517 / f64x8::splat(72.0) + t150 * t936 * t630 * t742 * t98 * t575 / f64x8::splat(144.0) + t150 * t936 * t328 * t749 * t98 * t575 / f64x8::splat(96.0) + t150 * t450 * t84 * t511 / f64x8::splat(72.0);
            let t975 = ((t2).select(f64x8::splat(0.0), -t6 * t54 * t539 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * (-t885 * t171 / f64x8::splat(24.0) - t885 * t176 / f64x8::splat(48.0) - t493 * t77 / f64x8::splat(48.0) + t892 * t436 / f64x8::splat(48.0) + t488 * t62 / f64x8::splat(24.0) - t150 * t899 * t42 * t98 * t575 / f64x8::splat(576.0) - t473 * t536 / f64x8::splat(216.0) - t463 * t524 / f64x8::splat(72.0) - t468 * t530 / f64x8::splat(72.0) - t150 * t899 * t328 * t329 * t98 * t575 / f64x8::splat(144.0) + t969)));
            let tv4rhosigma30 = f64x8::splat(2.0) * v_rho * t975 + f64x8::splat(2.0) * t543;
            acc_v4rhosigma3 = tv4rhosigma30;
            let t1006 = t589 * t572;
            let t1024 = t304 * v_sigma;
            let t1026 = t67 / t1024;
            let t1046 = t150 * t486 * t239;
            let t1051 = t150 * t486;
            let t1052 = t239 * t156;
            let t1081 = f64x8::splat(11.0) / f64x8::splat(4608.0) * t150 * t1006 * t84 * t225 * t20 * t603 - f64x8::splat(5.0) / f64x8::splat(128.0) * t65 * t1026 * t107 + t1046 * t171 / f64x8::splat(64.0) + t1046 * t176 / f64x8::splat(128.0) - t1051 * t1052 * t42 / f64x8::splat(128.0) + f64x8::splat(5.0) / f64x8::splat(64.0) * t22 * t25 / t26 / t1024 * t44 - t528 * t405 * t491 * t313 / f64x8::splat(384.0) - t534 * t412 * t491 * t313 / f64x8::splat(1152.0) - t522 * t398 * t491 * t313 / f64x8::splat(384.0) - t1051 * t1052 * t85 / f64x8::splat(64.0) + t509 * t491 * param_c * t313 / f64x8::splat(384.0);
            let t1086 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * (-t150 * t614 * t84 * t735 * t225 * t575 / f64x8::splat(1536.0) + t515 * t169 * t491 * t313 / f64x8::splat(384.0) - t150 * t614 * t630 * t742 * t225 * t575 / f64x8::splat(384.0) - t150 * t614 * t328 * t749 * t225 * t575 / f64x8::splat(256.0) - f64x8::splat(11.0) / f64x8::splat(4608.0) * t150 * t614 * t167 * t756 * t225 * t575 + t150 * t1006 * t42 * t225 * t575 / f64x8::splat(1536.0) + t150 * t1006 * t167 * t225 * t169 * t575 / f64x8::splat(256.0) + t150 * t1006 * t328 * t329 * t225 * t575 / f64x8::splat(384.0) + f64x8::splat(5.0) / f64x8::splat(128.0) * t65 * t1026 * t70 * t113 + t503 * t42 * t491 * t313 / f64x8::splat(1152.0) + t1081)));
            let tv4sigma40 = f64x8::splat(2.0) * v_rho * t1086;
            acc_v4sigma4 = tv4sigma40;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        store_add(v2rho2, ip, m, acc_v2rho2);
        store_add(v2rhosigma, ip, m, acc_v2rhosigma);
        store_add(v2sigma2, ip, m, acc_v2sigma2);
        store_add(v3rho3, ip, m, acc_v3rho3);
        store_add(v3rho2sigma, ip, m, acc_v3rho2sigma);
        store_add(v3rhosigma2, ip, m, acc_v3rhosigma2);
        store_add(v3sigma3, ip, m, acc_v3sigma3);
        store_add(v4rho4, ip, m, acc_v4rho4);
        store_add(v4rho3sigma, ip, m, acc_v4rho3sigma);
        store_add(v4rho2sigma2, ip, m, acc_v4rho2sigma2);
        store_add(v4rhosigma3, ip, m, acc_v4rhosigma3);
        store_add(v4sigma4, ip, m, acc_v4sigma4);
        ip += 8;
    }
}
