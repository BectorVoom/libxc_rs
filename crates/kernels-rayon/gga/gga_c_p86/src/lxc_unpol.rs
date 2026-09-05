//! GGA_C_P86 lxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_p86.c`
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
pub fn gga_c_p86_lxc_unpol(
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
    param_aa: f64,
    param_bb: f64,
    param_ftilde: f64,
    param_malpha: f64,
    param_mbeta: f64,
    param_mgamma: f64,
    param_mdelta: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_aa = f64x8::splat(param_aa);
    let param_bb = f64x8::splat(param_bb);
    let param_ftilde = f64x8::splat(param_ftilde);
    let param_malpha = f64x8::splat(param_malpha);
    let param_mbeta = f64x8::splat(param_mbeta);
    let param_mgamma = f64x8::splat(param_mgamma);
    let param_mdelta = f64x8::splat(param_mdelta);
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
            let t1 = f64x8::splat(M_CBRT3);
            let t2 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t3 = (simd::cbrt(t2));
            let t4 = t1 * t3;
            let t5 = f64x8::splat(M_CBRT4);
            let t6 = t5 * t5;
            let t7 = (simd::cbrt(v_rho));
            let t8 = f64x8::splat(1.0) / t7;
            let t9 = t6 * t8;
            let t10 = t4 * t9;
            let t11 = t10 / f64x8::splat(4.0);
            let t12 = (f64x8::splat(1.0)).simd_le(t11);
            let t13 = ((t10).sqrt());
            let t16 = f64x8::splat(1.0) + f64x8::splat(0.52645) * t13 + f64x8::splat(0.08335) * t10;
            let t19 = (simd::ln(t11));
            let t22 = t4 * t9 * t19;
            let t26 = ((t12).select(-f64x8::splat(0.1423) / t16, f64x8::splat(0.0311) * t19 - f64x8::splat(0.048) + f64x8::splat(0.0005) * t22 - f64x8::splat(0.0029) * t10));
            let t29 = f64x8::splat(1.0) + f64x8::splat(0.69905) * t13 + f64x8::splat(0.065275) * t10;
            let t36 = ((t12).select(-f64x8::splat(0.0843) / t29, f64x8::splat(0.01555) * t19 - f64x8::splat(0.0269) + f64x8::splat(0.000175) * t22 - f64x8::splat(0.0012) * t10));
            let t38 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t39 = (simd::cbrt(zeta_threshold));
            let t41 = ((t38).select(t39 * zeta_threshold, f64x8::splat(1.0)));
            let t43 = f64x8::splat(2.0) * t41 - f64x8::splat(2.0);
            let t45 = f64x8::splat(M_CBRT2);
            let t48 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t45 - f64x8::splat(2.0));
            let t49 = (t36 - t26) * t43 * t48;
            let t50 = v_rho * v_rho;
            let t52 = f64x8::splat(1.0) / t7 / t50;
            let t53 = v_sigma * t52;
            let t54 = param_aa + param_bb;
            let t55 = param_ftilde * t54;
            let t56 = param_malpha * t1;
            let t57 = t3 * t6;
            let t58 = t57 * t8;
            let t61 = t1 * t1;
            let t62 = param_mbeta * t61;
            let t63 = t3 * t3;
            let t64 = t63 * t5;
            let t65 = t7 * t7;
            let t66 = f64x8::splat(1.0) / t65;
            let t67 = t64 * t66;
            let t70 = param_bb + t56 * t58 / f64x8::splat(4.0) + t62 * t67 / f64x8::splat(4.0);
            let t71 = param_mgamma * t1;
            let t74 = param_mdelta * t61;
            let t77 = f64x8::splat(1.0) / v_rho;
            let t80 = f64x8::splat(1.0) + t71 * t58 / f64x8::splat(4.0) + t74 * t67 / f64x8::splat(4.0) + f64x8::splat(2387.32414637843) * param_mbeta * t77;
            let t81 = f64x8::splat(1.0) / t80;
            let t83 = t70 * t81 + param_aa;
            let t84 = f64x8::splat(1.0) / t83;
            let t85 = ((v_sigma).sqrt());
            let t86 = t84 * t85;
            let t87 = (simd::pow(v_rho, f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t89 = f64x8::splat(1.0) / t87 / v_rho;
            let t92 = (simd::exp(-t55 * t86 * t89));
            let t94 = t39 * t39;
            let t96 = ((t38).select(t94 * zeta_threshold, f64x8::splat(1.0)));
            let t97 = ((t96).sqrt());
            let t98 = f64x8::splat(1.0) / t97;
            let t99 = t92 * t83 * t98;
            let t100 = t53 * t99;
            let tzk0 = t26 + t49 + t100;
            acc_zk = tzk0;
            let t101 = t16 * t16;
            let t102 = f64x8::splat(1.0) / t101;
            let t104 = f64x8::splat(1.0) / t13 * t1;
            let t106 = f64x8::splat(1.0) / t7 / v_rho;
            let t107 = t57 * t106;
            let t108 = t104 * t107;
            let t110 = t6 * t106;
            let t111 = t4 * t110;
            let t113 = -f64x8::splat(0.08774166666666666) * t108 - f64x8::splat(0.027783333333333333) * t111;
            let t118 = t4 * t110 * t19;
            let t122 = ((t12).select(f64x8::splat(0.1423) * t102 * t113, -f64x8::splat(0.010366666666666666) * t77 - f64x8::splat(0.00016666666666666666) * t118 + f64x8::splat(0.0008) * t111));
            let t123 = t29 * t29;
            let t124 = f64x8::splat(1.0) / t123;
            let t127 = -f64x8::splat(0.11650833333333334) * t108 - f64x8::splat(0.021758333333333334) * t111;
            let t134 = ((t12).select(f64x8::splat(0.0843) * t124 * t127, -f64x8::splat(0.005183333333333333) * t77 - f64x8::splat(5.833333333333333e-05) * t118 + f64x8::splat(0.00034166666666666666) * t111));
            let t137 = (t134 - t122) * t43 * t48;
            let t138 = t50 * v_rho;
            let t140 = f64x8::splat(1.0) / t7 / t138;
            let t141 = v_sigma * t140;
            let t142 = t141 * t99;
            let t144 = t83 * t83;
            let t145 = f64x8::splat(1.0) / t144;
            let t146 = t55 * t145;
            let t147 = t85 * t89;
            let t152 = t64 / t65 / v_rho;
            let t155 = -t56 * t107 / f64x8::splat(12.0) - t62 * t152 / f64x8::splat(6.0);
            let t157 = t80 * t80;
            let t158 = f64x8::splat(1.0) / t157;
            let t159 = t70 * t158;
            let t164 = f64x8::splat(1.0) / t50;
            let t167 = -t71 * t107 / f64x8::splat(12.0) - t74 * t152 / f64x8::splat(6.0) - f64x8::splat(2387.32414637843) * param_mbeta * t164;
            let t169 = t155 * t81 - t159 * t167;
            let t173 = f64x8::splat(1.0) / t87 / t50;
            let t177 = t146 * t147 * t169 + f64x8::splat(7.0) / f64x8::splat(6.0) * t55 * t86 * t173;
            let t178 = t53 * t177;
            let t179 = t178 * t99;
            let t181 = t92 * t169 * t98;
            let t182 = t53 * t181;
            let tvrho0 = t26 + t49 + t100 + v_rho * (t122 + t137 - f64x8::splat(7.0) / f64x8::splat(3.0) * t142 + t179 + t182);
            acc_vrho = tvrho0;
            let t185 = t52 * t92;
            let t186 = t83 * t98;
            let t187 = t185 * t186;
            let t188 = ((v_rho).sqrt());
            let t190 = f64x8::splat(1.0) / t188 / t138;
            let t191 = t85 * t190;
            let t192 = t191 * param_ftilde;
            let t194 = t54 * t92 * t98;
            let t196 = t192 * t194 / f64x8::splat(2.0);
            let tvsigma0 = v_rho * (t187 - t196);
            acc_vsigma = tvsigma0;
            let t204 = f64x8::splat(1.0) / t101 / t16;
            let t205 = t113 * t113;
            let t210 = f64x8::splat(1.0) / t13 / t10 * t61;
            let t213 = t64 / t65 / t50;
            let t214 = t210 * t213;
            let t216 = t57 * t52;
            let t217 = t104 * t216;
            let t219 = t6 * t52;
            let t220 = t4 * t219;
            let t222 = -f64x8::splat(0.058494444444444446) * t214 + f64x8::splat(0.11698888888888889) * t217 + f64x8::splat(0.03704444444444444) * t220;
            let t228 = t4 * t219 * t19;
            let t232 = ((t12).select(-f64x8::splat(0.2846) * t204 * t205 + f64x8::splat(0.1423) * t102 * t222, f64x8::splat(0.010366666666666666) * t164 + f64x8::splat(0.00022222222222222223) * t228 - f64x8::splat(0.001011111111111111) * t220));
            let t234 = f64x8::splat(1.0) / t123 / t29;
            let t235 = t127 * t127;
            let t241 = -f64x8::splat(0.07767222222222223) * t214 + f64x8::splat(0.15534444444444445) * t217 + f64x8::splat(0.029011111111111113) * t220;
            let t249 = ((t12).select(-f64x8::splat(0.1686) * t234 * t235 + f64x8::splat(0.0843) * t124 * t241, f64x8::splat(0.005183333333333333) * t164 + f64x8::splat(7.777777777777778e-05) * t228 - f64x8::splat(0.00043611111111111113) * t220));
            let t252 = (t249 - t232) * t43 * t48;
            let t253 = t50 * t50;
            let t255 = f64x8::splat(1.0) / t7 / t253;
            let t256 = v_sigma * t255;
            let t257 = t256 * t99;
            let t259 = t141 * t177;
            let t260 = t259 * t99;
            let t262 = t141 * t181;
            let t265 = f64x8::splat(1.0) / t144 / t83;
            let t266 = t55 * t265;
            let t267 = t169 * t169;
            let t271 = t85 * t173;
            let t279 = t56 * t216 / f64x8::splat(9.0) + f64x8::splat(5.0) / f64x8::splat(18.0) * t62 * t213;
            let t281 = t155 * t158;
            let t285 = f64x8::splat(1.0) / t157 / t80;
            let t286 = t70 * t285;
            let t287 = t167 * t167;
            let t294 = f64x8::splat(1.0) / t138;
            let t297 = t71 * t216 / f64x8::splat(9.0) + f64x8::splat(5.0) / f64x8::splat(18.0) * t74 * t213 + f64x8::splat(4774.64829275686) * param_mbeta * t294;
            let t299 = -t159 * t297 - f64x8::splat(2.0) * t281 * t167 + t279 * t81 + f64x8::splat(2.0) * t286 * t287;
            let t303 = f64x8::splat(1.0) / t87 / t138;
            let t307 = -f64x8::splat(2.0) * t266 * t147 * t267 - f64x8::splat(7.0) / f64x8::splat(3.0) * t146 * t271 * t169 + t146 * t147 * t299 - f64x8::splat(91.0) / f64x8::splat(36.0) * t55 * t86 * t303;
            let t308 = t53 * t307;
            let t309 = t308 * t99;
            let t310 = t177 * t177;
            let t311 = t53 * t310;
            let t312 = t311 * t99;
            let t313 = t178 * t181;
            let t316 = t92 * t299 * t98;
            let t317 = t53 * t316;
            let tv2rho20 = f64x8::splat(2.0) * t122 + f64x8::splat(2.0) * t137 - f64x8::splat(14.0) / f64x8::splat(3.0) * t142 + f64x8::splat(2.0) * t179 + f64x8::splat(2.0) * t182 + v_rho * (t232 + t252 + f64x8::splat(70.0) / f64x8::splat(9.0) * t257 - f64x8::splat(14.0) / f64x8::splat(3.0) * t260 - f64x8::splat(14.0) / f64x8::splat(3.0) * t262 + t309 + t312 + f64x8::splat(2.0) * t313 + t317);
            acc_v2rho2 = tv2rho20;
            let t320 = t140 * t92;
            let t321 = t320 * t186;
            let t323 = t52 * t177;
            let t324 = t323 * t99;
            let t325 = t169 * t98;
            let t326 = t185 * t325;
            let t328 = f64x8::splat(1.0) / t188 / t253;
            let t330 = t85 * t328 * param_ftilde;
            let t331 = t330 * t194;
            let t334 = t92 * t98;
            let t335 = t54 * t177 * t334;
            let t336 = t192 * t335;
            let tv2rhosigma0 = t187 - t196 + v_rho * (-f64x8::splat(7.0) / f64x8::splat(3.0) * t321 + t324 + t326 + f64x8::splat(7.0) / f64x8::splat(4.0) * t331 - t336 / f64x8::splat(2.0));
            acc_v2rhosigma = tv2rhosigma0;
            let t341 = t190 * param_ftilde * t54;
            let t342 = f64x8::splat(1.0) / t85;
            let t343 = t342 * t92;
            let t344 = t343 * t98;
            let t346 = f64x8::splat(3.0) / f64x8::splat(4.0) * t341 * t344;
            let t348 = f64x8::splat(1.0) / t65 / t253;
            let t349 = param_ftilde * param_ftilde;
            let t350 = t348 * t349;
            let t351 = t54 * t54;
            let t352 = t350 * t351;
            let t354 = t84 * t92 * t98;
            let t356 = t352 * t354 / f64x8::splat(4.0);
            let tv2sigma20 = v_rho * (-t346 + t356);
            acc_v2sigma2 = tv2sigma20;
            let t367 = t123 * t123;
            let t368 = f64x8::splat(1.0) / t367;
            let t372 = t234 * t127;
            let t381 = f64x8::splat(1.0) / t13 / t61 / t63 / t5 / t66 * t2 / f64x8::splat(4.0);
            let t382 = f64x8::splat(1.0) / t253;
            let t383 = t381 * t382;
            let t387 = t64 / t65 / t138;
            let t388 = t210 * t387;
            let t390 = t57 * t140;
            let t391 = t104 * t390;
            let t393 = t6 * t140;
            let t394 = t4 * t393;
            let t396 = -f64x8::splat(0.46603333333333335) * t383 + f64x8::splat(0.3106888888888889) * t388 - f64x8::splat(0.3624703703703704) * t391 - f64x8::splat(0.0676925925925926) * t394;
            let t402 = t4 * t393 * t19;
            let t406 = ((t12).select(f64x8::splat(0.5058) * t368 * t235 * t127 - f64x8::splat(0.5058) * t372 * t241 + f64x8::splat(0.0843) * t124 * t396, -f64x8::splat(0.010366666666666666) * t294 - f64x8::splat(0.0001814814814814815) * t402 + f64x8::splat(0.0009916666666666667) * t394));
            let t407 = t101 * t101;
            let t408 = f64x8::splat(1.0) / t407;
            let t412 = t204 * t113;
            let t419 = -f64x8::splat(0.35096666666666665) * t383 + f64x8::splat(0.23397777777777778) * t388 - f64x8::splat(0.2729740740740741) * t391 - f64x8::splat(0.08643703703703703) * t394;
            let t427 = ((t12).select(f64x8::splat(0.8538) * t408 * t205 * t113 - f64x8::splat(0.8538) * t412 * t222 + f64x8::splat(0.1423) * t102 * t419, -f64x8::splat(0.020733333333333333) * t294 - f64x8::splat(0.0005185185185185185) * t402 + f64x8::splat(0.0022851851851851852) * t394));
            let t430 = (t406 - t427) * t43 * t48;
            let t431 = t256 * t181;
            let t433 = t141 * t316;
            let t439 = -f64x8::splat(7.0) / f64x8::splat(27.0) * t56 * t390 - f64x8::splat(20.0) / f64x8::splat(27.0) * t62 * t387;
            let t441 = t279 * t158;
            let t444 = t155 * t285;
            let t449 = t157 * t157;
            let t450 = f64x8::splat(1.0) / t449;
            let t451 = t70 * t450;
            let t452 = t287 * t167;
            let t455 = t167 * t297;
            let t464 = -f64x8::splat(7.0) / f64x8::splat(27.0) * t71 * t390 - f64x8::splat(20.0) / f64x8::splat(27.0) * t74 * t387 - f64x8::splat(14323.94487827058) * param_mbeta * t382;
            let t466 = -t159 * t464 - f64x8::splat(3.0) * t441 * t167 - f64x8::splat(3.0) * t281 * t297 + f64x8::splat(6.0) * t286 * t455 + f64x8::splat(6.0) * t444 * t287 + t439 * t81 - f64x8::splat(6.0) * t451 * t452;
            let t468 = t92 * t466 * t98;
            let t469 = t53 * t468;
            let t470 = t253 * v_rho;
            let t472 = f64x8::splat(1.0) / t7 / t470;
            let t473 = v_sigma * t472;
            let t474 = t473 * t99;
            let t476 = t256 * t177;
            let t477 = t476 * t99;
            let t479 = t141 * t307;
            let t480 = t479 * t99;
            let t482 = t259 * t181;
            let t484 = t144 * t144;
            let t486 = t55 / t484;
            let t487 = t267 * t169;
            let t494 = t169 * t299;
            let t498 = t85 * t303;
            let t508 = f64x8::splat(1.0) / t87 / t253;
            let t512 = f64x8::splat(6.0) * t486 * t147 * t487 + f64x8::splat(7.0) * t266 * t271 * t267 - f64x8::splat(6.0) * t266 * t147 * t494 + f64x8::splat(91.0) / f64x8::splat(12.0) * t146 * t498 * t169 - f64x8::splat(7.0) / f64x8::splat(2.0) * t146 * t271 * t299 + t146 * t147 * t466 + f64x8::splat(1729.0) / f64x8::splat(216.0) * t55 * t86 * t508;
            let t513 = t53 * t512;
            let t514 = t513 * t99;
            let t515 = t308 * t181;
            let t517 = t311 * t181;
            let t519 = t178 * t316;
            let t521 = t141 * t310;
            let t522 = t521 * t99;
            let t524 = t177 * t92;
            let t525 = t524 * t186;
            let t526 = t308 * t525;
            let t528 = t310 * t177;
            let t529 = t53 * t528;
            let t530 = t529 * t99;
            let t531 = t430 + f64x8::splat(70.0) / f64x8::splat(3.0) * t431 + t427 - f64x8::splat(7.0) * t433 + t469 - f64x8::splat(910.0) / f64x8::splat(27.0) * t474 + f64x8::splat(70.0) / f64x8::splat(3.0) * t477 - f64x8::splat(7.0) * t480 - f64x8::splat(14.0) * t482 + t514 + f64x8::splat(3.0) * t515 + f64x8::splat(3.0) * t517 + f64x8::splat(3.0) * t519 - f64x8::splat(7.0) * t522 + f64x8::splat(3.0) * t526 + t530;
            let tv3rho30 = f64x8::splat(3.0) * t232 + f64x8::splat(3.0) * t252 + f64x8::splat(70.0) / f64x8::splat(3.0) * t257 - f64x8::splat(14.0) * t260 - f64x8::splat(14.0) * t262 + f64x8::splat(3.0) * t309 + f64x8::splat(3.0) * t312 + f64x8::splat(6.0) * t313 + f64x8::splat(3.0) * t317 + v_rho * t531;
            acc_v3rho3 = tv3rho30;
            let t537 = t255 * t92;
            let t538 = t537 * t186;
            let t540 = t140 * t177;
            let t541 = t540 * t99;
            let t543 = t320 * t325;
            let t545 = t52 * t307;
            let t546 = t545 * t99;
            let t547 = t52 * t310;
            let t548 = t547 * t99;
            let t549 = t323 * t181;
            let t551 = t299 * t98;
            let t552 = t185 * t551;
            let t554 = f64x8::splat(1.0) / t188 / t470;
            let t556 = t85 * t554 * param_ftilde;
            let t557 = t556 * t194;
            let t559 = t330 * t335;
            let t562 = t54 * t307 * t334;
            let t563 = t192 * t562;
            let t566 = t54 * t310 * t334;
            let t567 = t192 * t566;
            let t569 = f64x8::splat(70.0) / f64x8::splat(9.0) * t538 - f64x8::splat(14.0) / f64x8::splat(3.0) * t541 - f64x8::splat(14.0) / f64x8::splat(3.0) * t543 + t546 + t548 + f64x8::splat(2.0) * t549 + t552 - f64x8::splat(63.0) / f64x8::splat(8.0) * t557 + f64x8::splat(7.0) / f64x8::splat(2.0) * t559 - t563 / f64x8::splat(2.0) - t567 / f64x8::splat(2.0);
            let tv3rho2sigma0 = -f64x8::splat(14.0) / f64x8::splat(3.0) * t321 + f64x8::splat(2.0) * t324 + f64x8::splat(2.0) * t326 + f64x8::splat(7.0) / f64x8::splat(2.0) * t331 - t336 + v_rho * t569;
            acc_v3rho2sigma = tv3rho2sigma0;
            let t572 = t328 * param_ftilde * t54;
            let t573 = t572 * t344;
            let t576 = t342 * t177 * t334;
            let t577 = t341 * t576;
            let t582 = f64x8::splat(1.0) / t65 / t470 * t349 * t351;
            let t583 = t582 * t354;
            let t585 = t145 * t92;
            let t586 = t585 * t325;
            let t587 = t352 * t586;
            let t590 = t84 * t177 * t334;
            let t591 = t352 * t590;
            let tv3rhosigma20 = -t346 + t356 + v_rho * (f64x8::splat(21.0) / f64x8::splat(8.0) * t573 - f64x8::splat(3.0) / f64x8::splat(4.0) * t577 - f64x8::splat(7.0) / f64x8::splat(6.0) * t583 - t587 / f64x8::splat(4.0) + t591 / f64x8::splat(4.0));
            acc_v3rhosigma2 = tv3rhosigma20;
            let t596 = f64x8::splat(1.0) / t85 / v_sigma;
            let t598 = t596 * t92 * t98;
            let t600 = f64x8::splat(3.0) / f64x8::splat(8.0) * t341 * t598;
            let t601 = f64x8::splat(1.0) / v_sigma;
            let t603 = t601 * t84 * t334;
            let t605 = f64x8::splat(3.0) / f64x8::splat(8.0) * t352 * t603;
            let t606 = t87 * t87;
            let t607 = t606 * t606;
            let t608 = t607 * t87;
            let t611 = t349 * param_ftilde;
            let t612 = f64x8::splat(1.0) / t608 / t470 * t611;
            let t613 = t351 * t54;
            let t614 = t612 * t613;
            let t616 = t145 * t342 * t334;
            let t618 = t614 * t616 / f64x8::splat(8.0);
            let tv3sigma30 = v_rho * (t600 + t605 - t618);
            acc_v3sigma3 = tv3sigma30;
            let t622 = t205 * t205;
            let t628 = t222 * t222;
            let t640 = f64x8::splat(1.0) / t13 / t77 * t472 * t4 * t6 / f64x8::splat(48.0);
            let t642 = f64x8::splat(1.0) / t470;
            let t643 = t381 * t642;
            let t645 = t64 * t348;
            let t646 = t210 * t645;
            let t648 = t57 * t255;
            let t649 = t104 * t648;
            let t651 = t6 * t255;
            let t652 = t4 * t651;
            let t660 = t4 * t651 * t19;
            let t664 = ((t12).select(-f64x8::splat(3.4152) / t407 / t16 * t622 + f64x8::splat(5.1228) * t408 * t205 * t222 - f64x8::splat(0.8538) * t204 * t628 - f64x8::splat(1.1384) * t412 * t419 + f64x8::splat(0.1423) * t102 * (-f64x8::splat(0.29247222222222224) * t640 + f64x8::splat(2.807733333333333) * t643 - f64x8::splat(1.0399012345679013) * t646 + f64x8::splat(0.9099135802469136) * t649 + f64x8::splat(0.28812345679012347) * t652), f64x8::splat(0.0622) * t382 + f64x8::splat(0.0017283950617283952) * t660 - f64x8::splat(0.0074444444444444445) * t652));
            let t673 = t235 * t235;
            let t679 = t241 * t241;
            let t697 = ((t12).select(-f64x8::splat(2.0232) / t367 / t29 * t673 + f64x8::splat(3.0348) * t368 * t235 * t241 - f64x8::splat(0.5058) * t234 * t679 - f64x8::splat(0.6744) * t372 * t396 + f64x8::splat(0.0843) * t124 * (-f64x8::splat(0.3883611111111111) * t640 + f64x8::splat(3.728266666666667) * t643 - f64x8::splat(1.3808395061728396) * t646 + f64x8::splat(1.2082345679012345) * t649 + f64x8::splat(0.22564197530864197) * t652), f64x8::splat(0.0311) * t382 + f64x8::splat(0.0006049382716049383) * t660 - f64x8::splat(0.0032450617283950618) * t652));
            let t709 = t307 * t307;
            let t713 = t310 * t310;
            let t747 = t287 * t287;
            let t753 = t297 * t297;
            let t767 = (f64x8::splat(70.0) / f64x8::splat(81.0) * t56 * t648 + f64x8::splat(220.0) / f64x8::splat(81.0) * t62 * t645) * t81 - f64x8::splat(4.0) * t439 * t158 * t167 + f64x8::splat(12.0) * t279 * t285 * t287 - f64x8::splat(6.0) * t441 * t297 - f64x8::splat(24.0) * t155 * t450 * t452 + f64x8::splat(24.0) * t444 * t455 - f64x8::splat(4.0) * t281 * t464 + f64x8::splat(24.0) * t70 / t449 / t80 * t747 - f64x8::splat(36.0) * t451 * t287 * t297 + f64x8::splat(6.0) * t286 * t753 + f64x8::splat(8.0) * t286 * t167 * t464 - t159 * (f64x8::splat(70.0) / f64x8::splat(81.0) * t71 * t648 + f64x8::splat(220.0) / f64x8::splat(81.0) * t74 * t645 + f64x8::splat(57295.77951308232) * param_mbeta * t642);
            let t771 = t664 - f64x8::splat(28.0) / f64x8::splat(3.0) * t141 * t468 + f64x8::splat(140.0) / f64x8::splat(3.0) * t256 * t316 - f64x8::splat(28.0) * t521 * t181 + (t697 - t664) * t43 * t48 + f64x8::splat(140.0) / f64x8::splat(3.0) * t256 * t310 * t99 - f64x8::splat(28.0) / f64x8::splat(3.0) * t141 * t528 * t99 + f64x8::splat(4.0) * t529 * t181 + f64x8::splat(3.0) * t53 * t709 * t99 + t53 * t713 * t99 + f64x8::splat(280.0) / f64x8::splat(3.0) * t476 * t181 - f64x8::splat(28.0) * t259 * t316 - f64x8::splat(28.0) / f64x8::splat(3.0) * t141 * t512 * t99 + t53 * t92 * t767 * t98;
            let t782 = t253 * t50;
            let t793 = t267 * t267;
            let t810 = t299 * t299;
            let t835 = -f64x8::splat(24.0) * t55 / t484 / t83 * t147 * t793 - f64x8::splat(28.0) * t486 * t271 * t487 + f64x8::splat(36.0) * t486 * t147 * t267 * t299 - f64x8::splat(91.0) / f64x8::splat(3.0) * t266 * t498 * t267 + f64x8::splat(28.0) * t266 * t271 * t494 - f64x8::splat(6.0) * t266 * t147 * t810 - f64x8::splat(8.0) * t266 * t147 * t169 * t466 - f64x8::splat(1729.0) / f64x8::splat(54.0) * t146 * t85 * t508 * t169 + f64x8::splat(91.0) / f64x8::splat(6.0) * t146 * t498 * t299 - f64x8::splat(14.0) / f64x8::splat(3.0) * t146 * t271 * t466 + t146 * t147 * t767 - f64x8::splat(43225.0) / f64x8::splat(1296.0) * t55 * t86 / t87 / t470;
            let t844 = t524 * t325;
            let t855 = -f64x8::splat(3640.0) / f64x8::splat(27.0) * t473 * t181 + f64x8::splat(140.0) / f64x8::splat(3.0) * t256 * t307 * t99 + f64x8::splat(4.0) * t178 * t468 - f64x8::splat(3640.0) / f64x8::splat(27.0) * t473 * t177 * t99 + f64x8::splat(14560.0) / f64x8::splat(81.0) * v_sigma / t7 / t782 * t99 - f64x8::splat(28.0) * t479 * t181 + t53 * t835 * t99 + f64x8::splat(6.0) * t308 * t316 + f64x8::splat(4.0) * t513 * t181 + f64x8::splat(6.0) * t311 * t316 + f64x8::splat(12.0) * t308 * t844 + f64x8::splat(4.0) * t513 * t525 - f64x8::splat(28.0) * t479 * t525 + f64x8::splat(6.0) * t308 * t310 * t92 * t186;
            let tv4rho40 = v_rho * (t771 + t855) + f64x8::splat(4.0) * t430 + f64x8::splat(280.0) / f64x8::splat(3.0) * t431 + f64x8::splat(4.0) * t427 - f64x8::splat(28.0) * t433 + f64x8::splat(4.0) * t469 - f64x8::splat(3640.0) / f64x8::splat(27.0) * t474 + f64x8::splat(280.0) / f64x8::splat(3.0) * t477 - f64x8::splat(28.0) * t480 - f64x8::splat(56.0) * t482 + f64x8::splat(4.0) * t514 + f64x8::splat(12.0) * t515 + f64x8::splat(12.0) * t517 + f64x8::splat(12.0) * t519 - f64x8::splat(28.0) * t522 + f64x8::splat(12.0) * t526 + f64x8::splat(4.0) * t530;
            acc_v4rho4 = tv4rho40;
            let t933 = -f64x8::splat(7.0) * t320 * t551 + f64x8::splat(3.0) * t545 * t177 * t99 + t52 * t528 * t99 - f64x8::splat(7.0) * t140 * t310 * t99 + f64x8::splat(693.0) / f64x8::splat(16.0) * t85 / t188 / t782 * param_ftilde * t194 - f64x8::splat(189.0) / f64x8::splat(8.0) * t556 * t335 + f64x8::splat(21.0) / f64x8::splat(4.0) * t330 * t562 - t192 * t54 * t512 * t334 / f64x8::splat(2.0) + f64x8::splat(21.0) / f64x8::splat(4.0) * t330 * t566 - f64x8::splat(3.0) / f64x8::splat(2.0) * t191 * t55 * t307 * t177 * t334 - t192 * t54 * t528 * t334 / f64x8::splat(2.0);
            let tv4rho3sigma0 = v_rho * (f64x8::splat(3.0) * t323 * t316 + f64x8::splat(3.0) * t547 * t181 + f64x8::splat(3.0) * t545 * t181 + t52 * t512 * t99 - f64x8::splat(14.0) * t540 * t181 - f64x8::splat(910.0) / f64x8::splat(27.0) * t472 * t92 * t186 + f64x8::splat(70.0) / f64x8::splat(3.0) * t255 * t177 * t99 - f64x8::splat(7.0) * t140 * t307 * t99 + t185 * t466 * t98 + f64x8::splat(70.0) / f64x8::splat(3.0) * t537 * t325 + t933) + f64x8::splat(6.0) * t549 + f64x8::splat(3.0) * t548 - f64x8::splat(3.0) / f64x8::splat(2.0) * t567 - f64x8::splat(189.0) / f64x8::splat(8.0) * t557 + f64x8::splat(21.0) / f64x8::splat(2.0) * t559 - f64x8::splat(3.0) / f64x8::splat(2.0) * t563 + f64x8::splat(70.0) / f64x8::splat(3.0) * t538 - f64x8::splat(14.0) * t541 + f64x8::splat(3.0) * t546 - f64x8::splat(14.0) * t543 + f64x8::splat(3.0) * t552;
            acc_v4rho3sigma = tv4rho3sigma0;
            let t996 = -f64x8::splat(189.0) / f64x8::splat(16.0) * t554 * param_ftilde * t54 * t344 + f64x8::splat(21.0) / f64x8::splat(4.0) * t572 * t576 - f64x8::splat(3.0) / f64x8::splat(4.0) * t341 * t342 * t307 * t334 - f64x8::splat(3.0) / f64x8::splat(4.0) * t341 * t342 * t310 * t334 + f64x8::splat(119.0) / f64x8::splat(18.0) / t65 / t782 * t349 * t351 * t354 + f64x8::splat(7.0) / f64x8::splat(3.0) * t582 * t586 - f64x8::splat(7.0) / f64x8::splat(3.0) * t582 * t590 + t352 * t265 * t92 * t98 * t267 / f64x8::splat(2.0) - t350 * t351 * t145 * t844 / f64x8::splat(2.0) - t352 * t585 * t551 / f64x8::splat(4.0) + t352 * t84 * t307 * t334 / f64x8::splat(4.0) + t352 * t84 * t310 * t334 / f64x8::splat(4.0);
            let tv4rho2sigma20 = f64x8::splat(21.0) / f64x8::splat(4.0) * t573 - f64x8::splat(3.0) / f64x8::splat(2.0) * t577 - f64x8::splat(7.0) / f64x8::splat(3.0) * t583 - t587 / f64x8::splat(2.0) + t591 / f64x8::splat(2.0) + v_rho * t996;
            acc_v4rho2sigma2 = tv4rho2sigma20;
            let t1007 = t350 * t351 * t601;
            let tv4rhosigma30 = t600 + t605 - t618 + v_rho * (-f64x8::splat(21.0) / f64x8::splat(16.0) * t572 * t598 + f64x8::splat(3.0) / f64x8::splat(8.0) * t341 * t596 * t177 * t334 - f64x8::splat(7.0) / f64x8::splat(4.0) * t582 * t603 - f64x8::splat(3.0) / f64x8::splat(8.0) * t1007 * t586 + f64x8::splat(3.0) / f64x8::splat(8.0) * t1007 * t590 + f64x8::splat(35.0) / f64x8::splat(48.0) / t608 / t782 * t611 * t613 * t616 + t612 * t613 * t265 * t343 * t325 / f64x8::splat(4.0) - t612 * t613 * t145 * t576 / f64x8::splat(8.0));
            acc_v4rhosigma3 = tv4rhosigma30;
            let t1029 = v_sigma * v_sigma;
            let t1047 = t349 * t349;
            let t1049 = t351 * t351;
            let tv4sigma40 = v_rho * (-f64x8::splat(9.0) / f64x8::splat(16.0) * t341 / t85 / t1029 * t92 * t98 - f64x8::splat(9.0) / f64x8::splat(16.0) * t352 / t1029 * t84 * t334 - t614 * t596 * t145 * t334 / f64x8::splat(8.0) + f64x8::splat(1.0) / t253 / t138 * t1047 * t1049 * t265 * t601 * t334 / f64x8::splat(16.0));
            acc_v4sigma4 = tv4sigma40;
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
        { let a: [f64; 8] = acc_v4rho4.into(); v4rho4[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v4rho3sigma.into(); v4rho3sigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v4rho2sigma2.into(); v4rho2sigma2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v4rhosigma3.into(); v4rhosigma3[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v4sigma4.into(); v4sigma4[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
