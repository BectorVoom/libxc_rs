//! GGA_C_PW91 fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_pw91.c`
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
pub fn gga_c_pw91_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
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
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_vsigma = V_ZERO;
        let mut acc_v2rho2 = V_ZERO;
        let mut acc_v2rhosigma = V_ZERO;
        let mut acc_v2sigma2 = V_ZERO;
        {
            let t1 = f64x8::splat(M_CBRT3);
            let t2 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t3 = (simd::cbrt(t2));
            let t4 = t1 * t3;
            let t5 = f64x8::splat(M_CBRT4);
            let t6 = t5 * t5;
            let t7 = (simd::cbrt(v_rho));
            let t10 = t4 * t6 / t7;
            let t12 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t10;
            let t13 = ((t10).sqrt());
            let t16 = ((t10) * (t10).sqrt());
            let t18 = t1 * t1;
            let t19 = t3 * t3;
            let t20 = t18 * t19;
            let t21 = t7 * t7;
            let t24 = t20 * t5 / t21;
            let t26 = f64x8::splat(3.79785) * t13 + f64x8::splat(0.8969) * t10 + f64x8::splat(0.204775) * t16 + f64x8::splat(0.123235) * t24;
            let t29 = f64x8::splat(1.0) + f64x8::splat(16.081824322151103) / t26;
            let t30 = (simd::ln(t29));
            let t32 = f64x8::splat(0.062182) * t12 * t30;
            let t33 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t34 = (simd::cbrt(zeta_threshold));
            let t36 = ((t33).select(t34 * zeta_threshold, f64x8::splat(1.0)));
            let t39 = f64x8::splat(M_CBRT2);
            let t43 = (f64x8::splat(2.0) * t36 - f64x8::splat(2.0)) / (f64x8::splat(2.0) * t39 - f64x8::splat(2.0));
            let t45 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t10;
            let t50 = f64x8::splat(5.1785) * t13 + f64x8::splat(0.905775) * t10 + f64x8::splat(0.1100325) * t16 + f64x8::splat(0.1241775) * t24;
            let t53 = f64x8::splat(1.0) + f64x8::splat(29.608574643216677) / t50;
            let t54 = (simd::ln(t53));
            let t57 = f64x8::splat(0.019751789702565206) * t43 * t45 * t54;
            let t58 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t59 = (simd::cbrt(t58));
            let t60 = t59 * t59;
            let t61 = t18 * t60;
            let t62 = t34 * t34;
            let t63 = ((t33).select(t62, f64x8::splat(1.0)));
            let t64 = t63 * t63;
            let t65 = t64 * t63;
            let t66 = f64x8::splat(1.0) / t59;
            let t67 = t18 * t66;
            let t68 = v_rho * v_rho;
            let t70 = f64x8::splat(1.0) / t7 / t68;
            let t72 = v_sigma * t70 * t39;
            let t73 = f64x8::splat(1.0) / t64;
            let t75 = f64x8::splat(1.0) / t3;
            let t76 = t75 * t5;
            let t77 = t73 * t18 * t76;
            let t83 = f64x8::splat(1.0) / t60;
            let t87 = (simd::exp(-f64x8::splat(128.97460341341235) * (-t32 + t57) / t65 * t1 * t83));
            let t88 = t87 - f64x8::splat(1.0);
            let t89 = f64x8::splat(1.0) / t88;
            let t90 = t66 * t89;
            let t91 = v_sigma * v_sigma;
            let t92 = t68 * t68;
            let t94 = f64x8::splat(1.0) / t21 / t92;
            let t95 = t91 * t94;
            let t97 = t39 * t39;
            let t98 = t64 * t64;
            let t99 = f64x8::splat(1.0) / t98;
            let t100 = t97 * t99;
            let t101 = f64x8::splat(1.0) / t19;
            let t102 = t101 * t6;
            let t103 = t100 * t102;
            let t106 = t72 * t77 / f64x8::splat(96.0) + f64x8::splat(0.0027166129655589867) * t90 * t95 * t103;
            let t107 = t1 * t66;
            let t109 = t107 * t89 * v_sigma;
            let t110 = t70 * t39;
            let t112 = t73 * t75 * t5;
            let t116 = t18 * t83;
            let t117 = t88 * t88;
            let t118 = f64x8::splat(1.0) / t117;
            let t119 = t118 * t91;
            let t120 = t116 * t119;
            let t121 = t94 * t97;
            let t122 = t99 * t101;
            let t123 = t122 * t6;
            let t124 = t121 * t123;
            let t127 = f64x8::splat(1.0) + f64x8::splat(0.08693161489788757) * t109 * t110 * t112 + f64x8::splat(0.0075571056687546295) * t120 * t124;
            let t128 = f64x8::splat(1.0) / t127;
            let t132 = f64x8::splat(1.0) + f64x8::splat(2.7818116767324024) * t67 * t106 * t128;
            let t133 = (simd::ln(t132));
            let t136 = f64x8::splat(0.002584488143490343) * t61 * t65 * t133;
            let t137 = t2 * t59;
            let t140 = f64x8::splat(2.568) + f64x8::splat(5.8165) * t10 + f64x8::splat(0.00184725) * t24;
            let t143 = f64x8::splat(1000.0) + f64x8::splat(2180.75) * t10 + f64x8::splat(118.0) * t24;
            let t144 = f64x8::splat(1.0) / t143;
            let t146 = t140 * t144 - f64x8::splat(0.0018535714285714286);
            let t147 = t146 * t63;
            let t149 = t137 * t147 * v_sigma;
            let t151 = (simd::cbrt(f64x8::splat(9.0)));
            let t152 = t151 * t151;
            let t156 = f64x8::splat(1.0) / t21 / t68;
            let t158 = v_sigma * t39;
            let t162 = (simd::exp(-f64x8::splat(25.0) / f64x8::splat(18.0) * t2 * t5 * t152 * t3 * t156 * t64 * t158));
            let t163 = t76 * t162;
            let t164 = t110 * t163;
            let t166 = t149 * t164 / f64x8::splat(2.0);
            let tzk0 = -t32 + t57 + t136 + t166;
            acc_zk = tzk0;
            let t168 = f64x8::splat(1.0) / t7 / v_rho;
            let t169 = t6 * t168;
            let t171 = t4 * t169 * t30;
            let t172 = f64x8::splat(0.0011073577833333333) * t171;
            let t173 = t26 * t26;
            let t174 = f64x8::splat(1.0) / t173;
            let t175 = t12 * t174;
            let t177 = f64x8::splat(1.0) / t13 * t1;
            let t178 = t3 * t6;
            let t179 = t178 * t168;
            let t180 = t177 * t179;
            let t182 = t4 * t169;
            let t184 = ((t10).sqrt());
            let t185 = t184 * t1;
            let t186 = t185 * t179;
            let t191 = t20 * t5 / t21 / v_rho;
            let t193 = -f64x8::splat(0.632975) * t180 - f64x8::splat(0.29896666666666666) * t182 - f64x8::splat(0.1023875) * t186 - f64x8::splat(0.08215666666666667) * t191;
            let t194 = f64x8::splat(1.0) / t29;
            let t195 = t193 * t194;
            let t196 = t175 * t195;
            let t197 = f64x8::splat(1.0) * t196;
            let t198 = t43 * t1;
            let t201 = t198 * t178 * t168 * t54;
            let t202 = f64x8::splat(0.0001831155503675316) * t201;
            let t203 = t43 * t45;
            let t204 = t50 * t50;
            let t205 = f64x8::splat(1.0) / t204;
            let t210 = -f64x8::splat(0.8630833333333333) * t180 - f64x8::splat(0.301925) * t182 - f64x8::splat(0.05501625) * t186 - f64x8::splat(0.082785) * t191;
            let t212 = f64x8::splat(1.0) / t53;
            let t213 = t205 * t210 * t212;
            let t214 = t203 * t213;
            let t215 = f64x8::splat(0.5848223397455204) * t214;
            let t216 = t68 * v_rho;
            let t218 = f64x8::splat(1.0) / t7 / t216;
            let t220 = v_sigma * t218 * t39;
            let t223 = t98 * t65;
            let t224 = f64x8::splat(1.0) / t223;
            let t225 = t121 * t224;
            let t226 = t119 * t225;
            let t227 = t172 + t197 - t202 - t215;
            let t229 = t227 * t1 * t87;
            let t230 = t102 * t229;
            let t233 = t92 * v_rho;
            let t235 = f64x8::splat(1.0) / t21 / t233;
            let t236 = t91 * t235;
            let t240 = -f64x8::splat(7.0) / f64x8::splat(288.0) * t220 * t77 + f64x8::splat(0.03550031648908154) * t226 * t230 - f64x8::splat(0.012677527172608605) * t90 * t236 * t103;
            let t244 = t127 * t127;
            let t245 = f64x8::splat(1.0) / t244;
            let t246 = t106 * t245;
            let t247 = t18 * t118;
            let t248 = t247 * t72;
            let t249 = t98 * t63;
            let t251 = f64x8::splat(1.0) / t249 * t75;
            let t252 = t5 * t227;
            let t254 = t251 * t252 * t87;
            let t257 = t218 * t39;
            let t262 = f64x8::splat(1.0) / t59 / t58;
            let t264 = f64x8::splat(1.0) / t117 / t88;
            let t265 = t262 * t264;
            let t267 = t265 * t95 * t97;
            let t268 = t224 * t101;
            let t269 = t6 * t227;
            let t271 = t268 * t269 * t87;
            let t274 = t235 * t97;
            let t275 = t274 * t123;
            let t278 = f64x8::splat(1.1360101276506094) * t248 * t254 - f64x8::splat(0.2028404347617377) * t109 * t257 * t112 + f64x8::splat(5.848048239485272) * t267 * t271 - f64x8::splat(0.03526649312085494) * t120 * t275;
            let t282 = f64x8::splat(2.7818116767324024) * t67 * t240 * t128 - f64x8::splat(2.7818116767324024) * t67 * t246 * t278;
            let t284 = f64x8::splat(1.0) / t132;
            let t286 = t61 * t65 * t282 * t284;
            let t290 = -f64x8::splat(1.9388333333333334) * t182 - f64x8::splat(0.0012315) * t191;
            let t292 = t143 * t143;
            let t293 = f64x8::splat(1.0) / t292;
            let t294 = t140 * t293;
            let t297 = -f64x8::splat(726.9166666666666) * t182 - f64x8::splat(78.66666666666667) * t191;
            let t299 = t290 * t144 - t294 * t297;
            let t300 = t299 * t63;
            let t302 = t137 * t300 * v_sigma;
            let t303 = t302 * t164;
            let t305 = t257 * t163;
            let t306 = t149 * t305;
            let t309 = f64x8::splat(1.0) / t58 * t59;
            let t310 = t146 * t65;
            let t312 = t309 * t310 * t91;
            let t313 = t92 * t68;
            let t314 = f64x8::splat(1.0) / t313;
            let t317 = t6 * t152 * t162;
            let t318 = t314 * t97 * t317;
            let t319 = t312 * t318;
            let tvrho0 = -t32 + t57 + t136 + t166 + v_rho * (t172 + t197 - t202 - t215 + f64x8::splat(0.002584488143490343) * t286 + t303 / f64x8::splat(2.0) - f64x8::splat(7.0) / f64x8::splat(6.0) * t306 + f64x8::splat(50.0) / f64x8::splat(27.0) * t319);
            acc_vrho = tvrho0;
            let t325 = t18 * t75 * t5;
            let t328 = v_sigma * t94;
            let t332 = t110 * t73 * t325 / f64x8::splat(96.0) + f64x8::splat(0.005433225931117973) * t90 * t328 * t103;
            let t339 = t39 * t73 * t76;
            let t342 = t118 * v_sigma;
            let t343 = t116 * t342;
            let t346 = f64x8::splat(0.08693161489788757) * t107 * t89 * t70 * t339 + f64x8::splat(0.015114211337509259) * t343 * t124;
            let t350 = f64x8::splat(2.7818116767324024) * t67 * t332 * t128 - f64x8::splat(2.7818116767324024) * t67 * t246 * t346;
            let t354 = f64x8::splat(0.002584488143490343) * t61 * t65 * t350 * t284;
            let t355 = t137 * t147;
            let t357 = t355 * t164 / f64x8::splat(2.0);
            let t360 = f64x8::splat(1.0) / t233;
            let t362 = t360 * t97 * t317;
            let t364 = f64x8::splat(25.0) / f64x8::splat(36.0) * t309 * t310 * v_sigma * t362;
            let tvsigma0 = v_rho * (t354 + t357 - t364);
            acc_vsigma = tvsigma0;
            let t373 = t6 * t70;
            let t375 = t4 * t373 * t30;
            let t376 = f64x8::splat(0.0014764770444444443) * t375;
            let t377 = t4 * t6;
            let t378 = t168 * t174;
            let t380 = t377 * t378 * t195;
            let t381 = f64x8::splat(0.035616666666666665) * t380;
            let t382 = t173 * t26;
            let t383 = f64x8::splat(1.0) / t382;
            let t384 = t12 * t383;
            let t385 = t193 * t193;
            let t386 = t385 * t194;
            let t387 = t384 * t386;
            let t388 = f64x8::splat(2.0) * t387;
            let t391 = f64x8::splat(1.0) / t13 / t10 * t18;
            let t392 = t19 * t5;
            let t393 = t392 * t156;
            let t394 = t391 * t393;
            let t396 = t178 * t70;
            let t397 = t177 * t396;
            let t399 = t4 * t373;
            let t401 = f64x8::splat(1.0)/((t10).sqrt());
            let t402 = t401 * t18;
            let t403 = t402 * t393;
            let t405 = t185 * t396;
            let t408 = t20 * t5 * t156;
            let t410 = -f64x8::splat(0.4219833333333333) * t394 + f64x8::splat(0.8439666666666666) * t397 + f64x8::splat(0.3986222222222222) * t399 + f64x8::splat(0.06825833333333334) * t403 + f64x8::splat(0.13651666666666668) * t405 + f64x8::splat(0.1369277777777778) * t408;
            let t411 = t410 * t194;
            let t412 = t175 * t411;
            let t413 = f64x8::splat(1.0) * t412;
            let t414 = t173 * t173;
            let t415 = f64x8::splat(1.0) / t414;
            let t416 = t12 * t415;
            let t417 = t29 * t29;
            let t418 = f64x8::splat(1.0) / t417;
            let t419 = t385 * t418;
            let t420 = t416 * t419;
            let t421 = f64x8::splat(16.081824322151103) * t420;
            let t424 = t198 * t178 * t70 * t54;
            let t425 = f64x8::splat(0.0002441540671567088) * t424;
            let t426 = t43 * t4;
            let t428 = t426 * t169 * t213;
            let t429 = f64x8::splat(0.010843580882781523) * t428;
            let t430 = t204 * t50;
            let t431 = f64x8::splat(1.0) / t430;
            let t432 = t210 * t210;
            let t434 = t431 * t432 * t212;
            let t435 = t203 * t434;
            let t436 = f64x8::splat(1.169644679491041) * t435;
            let t443 = -f64x8::splat(0.5753888888888888) * t394 + f64x8::splat(1.1507777777777777) * t397 + f64x8::splat(0.4025666666666667) * t399 + f64x8::splat(0.0366775) * t403 + f64x8::splat(0.073355) * t405 + f64x8::splat(0.137975) * t408;
            let t445 = t205 * t443 * t212;
            let t446 = t203 * t445;
            let t447 = f64x8::splat(0.5848223397455204) * t446;
            let t448 = t204 * t204;
            let t449 = f64x8::splat(1.0) / t448;
            let t450 = t449 * t432;
            let t451 = t53 * t53;
            let t452 = f64x8::splat(1.0) / t451;
            let t453 = t450 * t452;
            let t454 = t203 * t453;
            let t455 = f64x8::splat(17.315755899375862) * t454;
            let t457 = f64x8::splat(1.0) / t7 / t92;
            let t459 = v_sigma * t457 * t39;
            let t462 = t264 * t91;
            let t463 = t98 * t98;
            let t465 = f64x8::splat(1.0) / t463 / t64;
            let t466 = t121 * t465;
            let t467 = t462 * t466;
            let t468 = t227 * t227;
            let t469 = t102 * t468;
            let t470 = t87 * t87;
            let t472 = t18 * t470 * t83;
            let t473 = t469 * t472;
            let t476 = t274 * t224;
            let t477 = t119 * t476;
            let t480 = -t376 - t381 - t388 + t413 + t421 + t425 + t429 + t436 - t447 - t455;
            let t482 = t480 * t1 * t87;
            let t483 = t102 * t482;
            let t486 = t119 * t466;
            let t487 = t116 * t87;
            let t488 = t469 * t487;
            let t492 = f64x8::splat(1.0) / t21 / t313;
            let t493 = t91 * t492;
            let t497 = f64x8::splat(35.0) / f64x8::splat(432.0) * t459 * t77 + f64x8::splat(9.15727848045983) * t467 * t473 - f64x8::splat(0.3313362872314278) * t477 * t230 + f64x8::splat(0.03550031648908154) * t226 * t483 - f64x8::splat(4.578639240229915) * t486 * t488 + f64x8::splat(0.0718393206447821) * t90 * t493 * t103;
            let t501 = t240 * t245;
            let t506 = f64x8::splat(1.0) / t244 / t127;
            let t507 = t106 * t506;
            let t508 = t278 * t278;
            let t512 = t264 * v_sigma;
            let t513 = f64x8::splat(1.0) / t463;
            let t514 = t110 * t513;
            let t515 = t512 * t514;
            let t516 = t468 * t470;
            let t518 = t76 * t516 * t83;
            let t521 = t247 * t220;
            let t524 = t5 * t480;
            let t526 = t251 * t524 * t87;
            let t529 = t342 * t514;
            let t532 = t76 * t468 * t83 * t87;
            let t535 = t457 * t39;
            let t539 = t117 * t117;
            let t540 = f64x8::splat(1.0) / t539;
            let t541 = t540 * t91;
            let t542 = t541 * t466;
            let t543 = t516 * t1;
            let t544 = t102 * t543;
            let t548 = t265 * t236 * t97;
            let t551 = t6 * t480;
            let t553 = t268 * t551 * t87;
            let t557 = t468 * t1 * t87;
            let t558 = t102 * t557;
            let t561 = t492 * t97;
            let t562 = t561 * t123;
            let t565 = f64x8::splat(879.0987341241437) * t515 * t518 - f64x8::splat(5.301380595702844) * t521 * t254 + f64x8::splat(1.1360101276506094) * t248 * t526 - f64x8::splat(439.54936706207184) * t529 * t532 + f64x8::splat(0.6761347825391256) * t109 * t535 * t112 + f64x8::splat(23.22934218224557) * t542 * t544 - f64x8::splat(54.581783568529204) * t548 * t271 + f64x8::splat(5.848048239485272) * t267 * t553 - f64x8::splat(7.743114060748523) * t467 * t558 + f64x8::splat(0.199843461018178) * t120 * t562;
            let t569 = f64x8::splat(2.7818116767324024) * t67 * t497 * t128 - f64x8::splat(5.563623353464805) * t67 * t501 * t278 + f64x8::splat(5.563623353464805) * t67 * t507 * t508 - f64x8::splat(2.7818116767324024) * t67 * t246 * t565;
            let t572 = t61 * t65 * t569 * t284;
            let t574 = t282 * t282;
            let t576 = t132 * t132;
            let t577 = f64x8::splat(1.0) / t576;
            let t579 = t61 * t65 * t574 * t577;
            let t583 = f64x8::splat(2.585111111111111) * t399 + f64x8::splat(0.0020525) * t408;
            let t585 = t290 * t293;
            let t589 = f64x8::splat(1.0) / t292 / t143;
            let t590 = t140 * t589;
            let t591 = t297 * t297;
            let t596 = f64x8::splat(969.2222222222222) * t399 + f64x8::splat(131.11111111111111) * t408;
            let t598 = t583 * t144 - t294 * t596 - f64x8::splat(2.0) * t585 * t297 + f64x8::splat(2.0) * t590 * t591;
            let t599 = t598 * t63;
            let t601 = t137 * t599 * v_sigma;
            let t602 = t601 * t164;
            let t604 = t302 * t305;
            let t606 = t299 * t65;
            let t608 = t309 * t606 * t91;
            let t609 = t608 * t318;
            let t611 = t535 * t163;
            let t612 = t149 * t611;
            let t614 = t92 * t216;
            let t615 = f64x8::splat(1.0) / t614;
            let t617 = t615 * t97 * t317;
            let t618 = t312 * t617;
            let t622 = f64x8::splat(1.0) / t58 / f64x8::splat(M_PI) * t59;
            let t624 = t622 * t146 * t249;
            let t625 = t91 * v_sigma;
            let t626 = t92 * t92;
            let t627 = t626 * v_rho;
            let t629 = f64x8::splat(1.0) / t21 / t627;
            let t632 = t151 * t3 * t162;
            let t633 = t625 * t629 * t632;
            let t634 = t624 * t633;
            let t636 = -t376 - t381 - t388 + t413 + t421 + t425 + t429 + t436 - t447 - t455 + f64x8::splat(0.002584488143490343) * t572 - f64x8::splat(0.002584488143490343) * t579 + t602 / f64x8::splat(2.0) - f64x8::splat(7.0) / f64x8::splat(3.0) * t604 + f64x8::splat(100.0) / f64x8::splat(27.0) * t609 + f64x8::splat(35.0) / f64x8::splat(9.0) * t612 - f64x8::splat(1250.0) / f64x8::splat(81.0) * t618 + f64x8::splat(40000.0) / f64x8::splat(81.0) * t634;
            let tv2rho20 = f64x8::splat(0.0022147155666666666) * t171 + f64x8::splat(2.0) * t196 - f64x8::splat(0.0003662311007350632) * t201 - f64x8::splat(1.169644679491041) * t214 + f64x8::splat(0.005168976286980686) * t286 + t303 - f64x8::splat(7.0) / f64x8::splat(3.0) * t306 + f64x8::splat(100.0) / f64x8::splat(27.0) * t319 + v_rho * t636;
            acc_v2rho2 = tv2rho20;
            let t641 = t342 * t225;
            let t644 = v_sigma * t235;
            let t648 = -f64x8::splat(7.0) / f64x8::splat(288.0) * t257 * t73 * t325 + f64x8::splat(0.07100063297816309) * t641 * t230 - f64x8::splat(0.02535505434521721) * t90 * t644 * t103;
            let t652 = t332 * t245;
            let t659 = t67 * t106;
            let t660 = t506 * t346;
            let t661 = t660 * t278;
            let t664 = t247 * t110;
            let t672 = t265 * t328 * t97;
            let t677 = f64x8::splat(1.1360101276506094) * t664 * t254 - f64x8::splat(0.2028404347617377) * t107 * t89 * t218 * t339 + f64x8::splat(11.696096478970544) * t672 * t271 - f64x8::splat(0.07053298624170988) * t343 * t275;
            let t681 = f64x8::splat(2.7818116767324024) * t67 * t648 * t128 - f64x8::splat(2.7818116767324024) * t67 * t652 * t278 - f64x8::splat(2.7818116767324024) * t67 * t501 * t346 + f64x8::splat(5.563623353464805) * t659 * t661 - f64x8::splat(2.7818116767324024) * t67 * t246 * t677;
            let t684 = t61 * t65 * t681 * t284;
            let t686 = t61 * t65;
            let t687 = t350 * t577;
            let t689 = t686 * t687 * t282;
            let t691 = t137 * t300;
            let t692 = t691 * t164;
            let t694 = t355 * t305;
            let t701 = t97 * t6 * t152 * v_sigma * t162;
            let t702 = t309 * t310 * t314 * t701;
            let t706 = t309 * t606 * v_sigma * t362;
            let t709 = f64x8::splat(1.0) / t21 / t626;
            let t710 = t91 * t709;
            let t711 = t710 * t632;
            let t712 = t624 * t711;
            let tv2rhosigma0 = t354 + t357 - t364 + v_rho * (f64x8::splat(0.002584488143490343) * t684 - f64x8::splat(0.002584488143490343) * t689 + t692 / f64x8::splat(2.0) - f64x8::splat(7.0) / f64x8::splat(6.0) * t694 + f64x8::splat(575.0) / f64x8::splat(108.0) * t702 - f64x8::splat(25.0) / f64x8::splat(36.0) * t706 - f64x8::splat(5000.0) / f64x8::splat(27.0) * t712);
            acc_v2rhosigma = tv2rhosigma0;
            let t716 = t89 * t94;
            let t719 = t100 * t102 * t128;
            let t725 = t346 * t346;
            let t729 = t1 * t106;
            let t730 = t245 * t118;
            let t731 = t729 * t730;
            let t734 = f64x8::splat(0.015114211337509259) * t116 * t716 * t719 - f64x8::splat(5.563623353464805) * t67 * t652 * t346 + f64x8::splat(5.563623353464805) * t67 * t507 * t725 - f64x8::splat(0.012780113936069357) * t731 * t124;
            let t738 = f64x8::splat(0.002584488143490343) * t61 * t65 * t734 * t284;
            let t739 = t350 * t350;
            let t743 = f64x8::splat(0.002584488143490343) * t61 * t65 * t739 * t577;
            let t744 = t309 * t310;
            let t746 = f64x8::splat(25.0) / f64x8::splat(18.0) * t744 * t362;
            let t748 = f64x8::splat(1.0) / t21 / t614;
            let t749 = v_sigma * t748;
            let t750 = t749 * t632;
            let t752 = f64x8::splat(625.0) / f64x8::splat(9.0) * t624 * t750;
            let tv2sigma20 = v_rho * (t738 - t743 - t746 + t752);
            acc_v2sigma2 = tv2sigma20;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        store_add(v2rho2, ip, m, acc_v2rho2);
        store_add(v2rhosigma, ip, m, acc_v2rhosigma);
        store_add(v2sigma2, ip, m, acc_v2sigma2);
        ip += 8;
    }
}
