//! MGGA_C_REVSCAN vxc unpol kernel — explicit SIMD (exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_revscan.c`
//! by tools/translate_rayon/from_maple.py, then rewritten to
//! `wide::f64x8` by simd.py (exact math). Eight grid points per step; every lane runs maple2c's expression
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
pub fn mgga_c_revscan_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let v_lapl = load(lapl, ip, np);
        let v_tau = load(tau, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_vsigma = V_ZERO;
        let mut acc_vlapl = V_ZERO;
        let mut acc_vtau = V_ZERO;
        {
            let t2 = f64x8::splat(M_CBRT3);
            let t3 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t4 = (simd::cbrt(t3));
            let t5 = t2 * t4;
            let t6 = f64x8::splat(M_CBRT4);
            let t7 = t6 * t6;
            let t8 = (simd::cbrt(v_rho));
            let t11 = t5 * t7 / t8;
            let t13 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t11;
            let t14 = ((t11).sqrt());
            let t17 = ((t11) * (t11).sqrt());
            let t19 = t2 * t2;
            let t20 = t4 * t4;
            let t21 = t19 * t20;
            let t22 = t8 * t8;
            let t25 = t21 * t6 / t22;
            let t27 = f64x8::splat(3.79785) * t14 + f64x8::splat(0.8969) * t11 + f64x8::splat(0.204775) * t17 + f64x8::splat(0.123235) * t25;
            let t30 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t27;
            let t31 = (simd::ln(t30));
            let t33 = f64x8::splat(0.0621814) * t13 * t31;
            let t34 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t35 = (simd::cbrt(zeta_threshold));
            let t37 = ((t34).select(t35 * zeta_threshold, f64x8::splat(1.0)));
            let t39 = f64x8::splat(2.0) * t37 - f64x8::splat(2.0);
            let t40 = f64x8::splat(M_CBRT2);
            let t41 = t40 - f64x8::splat(1.0);
            let t43 = f64x8::splat(1.0) / t41 / f64x8::splat(2.0);
            let t44 = t39 * t43;
            let t46 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t11;
            let t51 = f64x8::splat(5.1785) * t14 + f64x8::splat(0.905775) * t11 + f64x8::splat(0.1100325) * t17 + f64x8::splat(0.1241775) * t25;
            let t54 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t51;
            let t55 = (simd::ln(t54));
            let t58 = f64x8::splat(0.0197516734986138) * t44 * t46 * t55;
            let t59 = (simd::ln(f64x8::splat(2.0)));
            let t60 = f64x8::splat(1.0) - t59;
            let t61 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t63 = t60 / t61;
            let t64 = t35 * t35;
            let t65 = ((t34).select(t64, f64x8::splat(1.0)));
            let t66 = t65 * t65;
            let t67 = t66 * t65;
            let t69 = f64x8::splat(1.0) + f64x8::splat(0.025) * t11;
            let t71 = f64x8::splat(1.0) + f64x8::splat(0.04445) * t11;
            let t72 = f64x8::splat(1.0) / t71;
            let t73 = t69 * t72;
            let t74 = f64x8::splat(1.0) / t60;
            let t77 = f64x8::splat(1.0) / t67;
            let t78 = t61 * t77;
            let t80 = (simd::exp(-(-t33 + t58) * t74 * t78));
            let t81 = t80 - f64x8::splat(1.0);
            let t82 = f64x8::splat(1.0) / t81;
            let t83 = t74 * t82;
            let t84 = t83 * v_sigma;
            let t85 = t73 * t84;
            let t86 = v_rho * v_rho;
            let t88 = f64x8::splat(1.0) / t8 / t86;
            let t89 = t88 * t40;
            let t90 = f64x8::splat(1.0) / t66;
            let t92 = f64x8::splat(1.0) / t4;
            let t93 = t19 * t92;
            let t94 = t93 * t6;
            let t95 = t89 * t90 * t94;
            let t98 = f64x8::splat(1.0) + f64x8::splat(0.054878743191129266) * t85 * t95;
            let t99 = ((t98).sqrt().sqrt());
            let t102 = t69 * t69;
            let t103 = t71 * t71;
            let t104 = f64x8::splat(1.0) / t103;
            let t105 = t102 * t104;
            let t106 = t60 * t60;
            let t107 = f64x8::splat(1.0) / t106;
            let t108 = t81 * t81;
            let t109 = f64x8::splat(1.0) / t108;
            let t110 = t107 * t109;
            let t111 = v_sigma * v_sigma;
            let t112 = t110 * t111;
            let t113 = t105 * t112;
            let t114 = t86 * t86;
            let t116 = f64x8::splat(1.0) / t22 / t114;
            let t117 = t40 * t40;
            let t118 = t116 * t117;
            let t119 = t66 * t66;
            let t120 = f64x8::splat(1.0) / t119;
            let t121 = t118 * t120;
            let t122 = f64x8::splat(1.0) / t20;
            let t123 = t2 * t122;
            let t124 = t123 * t7;
            let t125 = t121 * t124;
            let t128 = f64x8::splat(1.0) + f64x8::splat(0.011293786703392187) * t113 * t125;
            let t129 = (simd::pow(t128, f64x8::splat(1.0) / f64x8::splat(8.0)));
            let t132 = f64x8::splat(1.0) - f64x8::splat(1.0) / t99 / f64x8::splat(2.0) - f64x8::splat(1.0) / t129 / f64x8::splat(2.0);
            let t135 = f64x8::splat(1.0) + f64x8::splat(1.0) * t132 * t81;
            let t136 = (simd::ln(t135));
            let t138 = t63 * t67 * t136;
            let t140 = f64x8::splat(1.0) / t22 / v_rho;
            let t143 = f64x8::splat(1.0) / t22 / t86;
            let t147 = f64x8::splat(M_CBRT6);
            let t149 = (simd::cbrt(t61));
            let t150 = t149 * t149;
            let t151 = f64x8::splat(1.0) / t150;
            let t152 = t151 * t117;
            let t154 = f64x8::splat(5.0) / f64x8::splat(9.0) * (v_tau * t140 - v_sigma * t143 / f64x8::splat(8.0)) * t147 * t152;
            let t155 = (t154).simd_le(f64x8::splat(1.0));
            let t156 = (simd::ln(f64x8::splat(f64::EPSILON)));
            let t159 = t156 / (-t156 + f64x8::splat(1.131));
            let t160 = (-t159).simd_lt(t154);
            let t161 = (t154).simd_lt(-t159);
            let t162 = ((t161).select(t154, -t159));
            let t163 = f64x8::splat(1.0) - t162;
            let t164 = f64x8::splat(1.0) / t163;
            let t167 = (simd::exp(-f64x8::splat(1.131) * t162 * t164));
            let t168 = ((t160).select(f64x8::splat(0.0), t167));
            let t170 = (simd::ln(f64x8::splat(0.7299270072992701) * f64x8::splat(f64::EPSILON)));
            let t173 = (-t170 + f64x8::splat(1.7)) / t170;
            let t174 = (t154).simd_lt(-t173);
            let t175 = ((t174).select(-t173, t154));
            let t176 = f64x8::splat(1.0) - t175;
            let t179 = (simd::exp(f64x8::splat(1.7) / t176));
            let t181 = ((t174).select(f64x8::splat(0.0), -f64x8::splat(1.37) * t179));
            let t182 = ((t155).select(t168, t181));
            let t185 = f64x8::splat(1.0) + f64x8::splat(0.033115) * t14 + f64x8::splat(0.04168) * t11;
            let t186 = f64x8::splat(1.0) / t185;
            let t189 = (simd::exp(f64x8::splat(1.0) * t186));
            let t190 = t189 - f64x8::splat(1.0);
            let t191 = t147 * t151;
            let t192 = t117 * v_sigma;
            let t196 = f64x8::splat(1.0) + f64x8::splat(0.04267528420875272) * t191 * t192 * t143;
            let t197 = ((t196).sqrt().sqrt());
            let t200 = t147 * t147;
            let t202 = f64x8::splat(1.0) / t149 / t61;
            let t203 = t200 * t202;
            let t204 = t40 * t111;
            let t205 = t114 * v_rho;
            let t207 = f64x8::splat(1.0) / t8 / t205;
            let t211 = f64x8::splat(1.0) + f64x8::splat(0.004552949705744548) * t203 * t204 * t207;
            let t212 = (simd::pow(t211, f64x8::splat(1.0) / f64x8::splat(8.0)));
            let t215 = f64x8::splat(1.0) - f64x8::splat(1.0) / t197 / f64x8::splat(2.0) - f64x8::splat(1.0) / t212 / f64x8::splat(2.0);
            let t217 = t190 * t215 + f64x8::splat(1.0);
            let t218 = (simd::ln(t217));
            let t224 = f64x8::splat(1.0) - f64x8::splat(2.363) * t41 * t39 * t43;
            let t226 = (-f64x8::splat(0.030197) * t186 + f64x8::splat(0.030197) * t218) * t224 + t33 - t58 - t138;
            let t227 = t182 * t226;
            let tzk0 = -t33 + t58 + t138 + t227;
            acc_zk = tzk0;
            let t229 = f64x8::splat(1.0) / t8 / v_rho;
            let t230 = t7 * t229;
            let t232 = t5 * t230 * t31;
            let t233 = f64x8::splat(0.0011073470983333333) * t232;
            let t234 = t27 * t27;
            let t235 = f64x8::splat(1.0) / t234;
            let t236 = t13 * t235;
            let t238 = f64x8::splat(1.0) / t14 * t2;
            let t239 = t4 * t7;
            let t240 = t239 * t229;
            let t241 = t238 * t240;
            let t243 = t5 * t230;
            let t245 = ((t11).sqrt());
            let t246 = t245 * t2;
            let t247 = t246 * t240;
            let t250 = t21 * t6 * t140;
            let t252 = -f64x8::splat(0.632975) * t241 - f64x8::splat(0.29896666666666666) * t243 - f64x8::splat(0.1023875) * t247 - f64x8::splat(0.08215666666666667) * t250;
            let t253 = f64x8::splat(1.0) / t30;
            let t254 = t252 * t253;
            let t255 = t236 * t254;
            let t256 = f64x8::splat(1.0) * t255;
            let t257 = t44 * t2;
            let t260 = t257 * t239 * t229 * t55;
            let t261 = f64x8::splat(0.00018311447306006544) * t260;
            let t262 = t44 * t46;
            let t263 = t51 * t51;
            let t264 = f64x8::splat(1.0) / t263;
            let t269 = -f64x8::splat(0.8630833333333333) * t241 - f64x8::splat(0.301925) * t243 - f64x8::splat(0.05501625) * t247 - f64x8::splat(0.082785) * t250;
            let t271 = f64x8::splat(1.0) / t54;
            let t272 = t264 * t269 * t271;
            let t273 = t262 * t272;
            let t274 = f64x8::splat(0.5848223622634646) * t273;
            let t276 = f64x8::splat(1.0) / t99 / t98;
            let t277 = t86 * v_rho;
            let t279 = f64x8::splat(1.0) / t22 / t277;
            let t280 = t279 * t72;
            let t283 = t40 * t90;
            let t284 = t82 * v_sigma * t283;
            let t287 = t69 * t104;
            let t288 = t287 * t83;
            let t289 = v_sigma * t279;
            let t293 = t73 * t107;
            let t294 = t109 * v_sigma;
            let t295 = t294 * t89;
            let t296 = t293 * t295;
            let t297 = t119 * t65;
            let t298 = f64x8::splat(1.0) / t297;
            let t299 = t298 * t19;
            let t300 = t299 * t92;
            let t301 = t233 + t256 - t261 - t274;
            let t302 = t6 * t301;
            let t303 = t61 * t80;
            let t304 = t302 * t303;
            let t305 = t300 * t304;
            let t309 = f64x8::splat(1.0) / t8 / t277;
            let t310 = t309 * t40;
            let t312 = t310 * t90 * t94;
            let t315 = -f64x8::splat(0.005487874319112926) * t280 * t74 * t284 + f64x8::splat(0.009757440539382782) * t288 * t289 * t283 + f64x8::splat(0.054878743191129266) * t296 * t305 - f64x8::splat(0.1280504007793016) * t85 * t312;
            let t319 = f64x8::splat(1.0) / t129 / t128;
            let t320 = t287 * t112;
            let t321 = t114 * t86;
            let t322 = f64x8::splat(1.0) / t321;
            let t323 = t322 * t117;
            let t324 = t323 * t120;
            let t325 = t324 * t94;
            let t328 = t103 * t71;
            let t329 = f64x8::splat(1.0) / t328;
            let t330 = t102 * t329;
            let t331 = t330 * t112;
            let t334 = t106 * t60;
            let t335 = f64x8::splat(1.0) / t334;
            let t336 = t105 * t335;
            let t337 = t108 * t81;
            let t338 = f64x8::splat(1.0) / t337;
            let t339 = t338 * t111;
            let t340 = t339 * t118;
            let t341 = t336 * t340;
            let t342 = t119 * t67;
            let t343 = f64x8::splat(1.0) / t342;
            let t344 = t343 * t2;
            let t345 = t344 * t122;
            let t347 = t7 * t301 * t303;
            let t348 = t345 * t347;
            let t352 = f64x8::splat(1.0) / t22 / t205;
            let t353 = t352 * t117;
            let t354 = t353 * t120;
            let t355 = t354 * t124;
            let t358 = -f64x8::splat(0.0007529191135594791) * t320 * t325 + f64x8::splat(0.001338690183908754) * t331 * t325 + f64x8::splat(0.022587573406784373) * t341 * t348 - f64x8::splat(0.052704337949163536) * t113 * t355;
            let t361 = t276 * t315 / f64x8::splat(8.0) + t319 * t358 / f64x8::splat(16.0);
            let t366 = t78 * t80;
            let t369 = f64x8::splat(1.0) * t361 * t81 - f64x8::splat(1.0) * t132 * t301 * t74 * t366;
            let t371 = f64x8::splat(1.0) / t135;
            let t373 = t63 * t67 * t369 * t371;
            let t380 = f64x8::splat(5.0) / f64x8::splat(9.0) * (-f64x8::splat(5.0) / f64x8::splat(3.0) * v_tau * t143 + t289 / f64x8::splat(3.0)) * t147 * t152;
            let t381 = ((t161).select(t380, f64x8::splat(0.0)));
            let t384 = t163 * t163;
            let t385 = f64x8::splat(1.0) / t384;
            let t386 = t162 * t385;
            let t389 = -f64x8::splat(1.131) * t381 * t164 - f64x8::splat(1.131) * t386 * t381;
            let t390 = t389 * t167;
            let t391 = ((t160).select(f64x8::splat(0.0), t390));
            let t392 = t176 * t176;
            let t393 = f64x8::splat(1.0) / t392;
            let t394 = ((t174).select(f64x8::splat(0.0), t380));
            let t398 = ((t174).select(f64x8::splat(0.0), -f64x8::splat(2.329) * t393 * t394 * t179));
            let t399 = ((t155).select(t391, t398));
            let t400 = t399 * t226;
            let t401 = t185 * t185;
            let t402 = f64x8::splat(1.0) / t401;
            let t405 = -f64x8::splat(0.005519166666666667) * t241 - f64x8::splat(0.013893333333333334) * t243;
            let t406 = t402 * t405;
            let t408 = t189 * t215;
            let t412 = f64x8::splat(1.0) / t197 / t196;
            let t414 = t412 * t147 * t151;
            let t420 = f64x8::splat(1.0) / t212 / t211 * t200;
            let t421 = t420 * t202;
            let t423 = f64x8::splat(1.0) / t8 / t321;
            let t427 = -f64x8::splat(0.014225094736250906) * t414 * t192 * t279 - f64x8::splat(0.001517649901914849) * t421 * t204 * t423;
            let t429 = -f64x8::splat(1.0) * t406 * t408 + t190 * t427;
            let t430 = f64x8::splat(1.0) / t217;
            let t435 = (f64x8::splat(0.030197) * t406 + f64x8::splat(0.030197) * t429 * t430) * t224 - t233 - t256 + t261 + t274 - t373;
            let t436 = t182 * t435;
            let tvrho0 = -t33 + t58 + t138 + t227 + v_rho * (t233 + t256 - t261 - t274 + t373 + t400 + t436);
            acc_vrho = tvrho0;
            let t439 = t60 * t67;
            let t440 = t276 * t69;
            let t441 = t72 * t74;
            let t443 = t440 * t441 * t82;
            let t446 = t319 * t102;
            let t447 = t446 * t104;
            let t448 = t110 * v_sigma;
            let t449 = t447 * t448;
            let t452 = f64x8::splat(0.006859842898891158) * t443 * t95 + f64x8::splat(0.0014117233379240233) * t449 * t125;
            let t453 = t452 * t81;
            let t456 = f64x8::splat(0.10132118364233778) * t439 * t453 * t371;
            let t458 = t143 * t147 * t152;
            let t459 = f64x8::splat(5.0) / f64x8::splat(72.0) * t458;
            let t460 = ((t161).select(-t459, f64x8::splat(0.0)));
            let t465 = -f64x8::splat(1.131) * t460 * t164 - f64x8::splat(1.131) * t386 * t460;
            let t466 = t465 * t167;
            let t467 = ((t160).select(f64x8::splat(0.0), t466));
            let t468 = ((t174).select(f64x8::splat(0.0), -t459));
            let t472 = ((t174).select(f64x8::splat(0.0), -f64x8::splat(2.329) * t393 * t468 * t179));
            let t473 = ((t155).select(t467, t472));
            let t474 = t473 * t226;
            let t476 = t191 * t117;
            let t479 = t40 * v_sigma;
            let t483 = f64x8::splat(0.00533441052609409) * t412 * t143 * t476 + f64x8::splat(0.0005691187132180684) * t421 * t479 * t207;
            let t484 = t190 * t483;
            let t485 = t430 * t224;
            let t488 = f64x8::splat(0.030197) * t484 * t485 - t456;
            let t489 = t182 * t488;
            let tvsigma0 = v_rho * (t456 + t474 + t489);
            acc_vsigma = tvsigma0;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl = tvlapl0;
            let t493 = f64x8::splat(5.0) / f64x8::splat(9.0) * t140 * t147 * t152;
            let t494 = ((t161).select(t493, f64x8::splat(0.0)));
            let t499 = -f64x8::splat(1.131) * t494 * t164 - f64x8::splat(1.131) * t386 * t494;
            let t500 = t499 * t167;
            let t501 = ((t160).select(f64x8::splat(0.0), t500));
            let t502 = ((t174).select(f64x8::splat(0.0), t493));
            let t506 = ((t174).select(f64x8::splat(0.0), -f64x8::splat(2.329) * t393 * t502 * t179));
            let t507 = ((t155).select(t501, t506));
            let t508 = v_rho * t507;
            let tvtau0 = t508 * t226;
            acc_vtau = tvtau0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vlapl.into(); vlapl[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vtau.into(); vtau[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
