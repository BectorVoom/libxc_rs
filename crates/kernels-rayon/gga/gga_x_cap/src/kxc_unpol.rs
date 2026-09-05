//! GGA_X_CAP kxc unpol kernel — explicit SIMD (bit-exact).
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

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_cap_kxc_unpol(
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
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rho2.into(); v2rho2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rhosigma.into(); v2rhosigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2sigma2.into(); v2sigma2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v3rho3.into(); v3rho3[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v3rho2sigma.into(); v3rho2sigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v3rhosigma2.into(); v3rhosigma2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v3sigma3.into(); v3sigma3[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
