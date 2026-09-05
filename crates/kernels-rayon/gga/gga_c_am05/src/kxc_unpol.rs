//! GGA_C_AM05 kxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_am05.c`
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
pub fn gga_c_am05_kxc_unpol(
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
    param_alpha: f64,
    param_gamma: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_alpha = f64x8::splat(param_alpha);
    let param_gamma = f64x8::splat(param_gamma);
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
            let t29 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t26;
            let t30 = (simd::ln(t29));
            let t33 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t34 = (simd::cbrt(zeta_threshold));
            let t36 = ((t33).select(t34 * zeta_threshold, f64x8::splat(1.0)));
            let t39 = f64x8::splat(M_CBRT2);
            let t43 = (f64x8::splat(2.0) * t36 - f64x8::splat(2.0)) / (f64x8::splat(2.0) * t39 - f64x8::splat(2.0));
            let t45 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t10;
            let t50 = f64x8::splat(5.1785) * t13 + f64x8::splat(0.905775) * t10 + f64x8::splat(0.1100325) * t16 + f64x8::splat(0.1241775) * t24;
            let t53 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t50;
            let t54 = (simd::ln(t53));
            let t58 = -f64x8::splat(0.0621814) * t12 * t30 + f64x8::splat(0.0197516734986138) * t43 * t45 * t54;
            let t59 = ((t33).select(zeta_threshold, f64x8::splat(1.0)));
            let t60 = t58 * t59;
            let t61 = f64x8::splat(M_CBRT6);
            let t62 = param_alpha * t61;
            let t63 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t64 = (simd::cbrt(t63));
            let t65 = t64 * t64;
            let t66 = f64x8::splat(1.0) / t65;
            let t68 = t39 * t39;
            let t69 = v_sigma * t68;
            let t70 = v_rho * v_rho;
            let t72 = f64x8::splat(1.0) / t21 / t70;
            let t76 = f64x8::splat(1.0) + t62 * t66 * t69 * t72 / f64x8::splat(24.0);
            let t77 = f64x8::splat(1.0) / t76;
            let t80 = t77 + param_gamma * (f64x8::splat(1.0) - t77);
            let tzk0 = t60 * t80;
            acc_zk = tzk0;
            let t82 = f64x8::splat(1.0) / t7 / v_rho;
            let t83 = t6 * t82;
            let t87 = t26 * t26;
            let t88 = f64x8::splat(1.0) / t87;
            let t89 = t12 * t88;
            let t91 = f64x8::splat(1.0) / t13 * t1;
            let t92 = t3 * t6;
            let t93 = t92 * t82;
            let t94 = t91 * t93;
            let t96 = t4 * t83;
            let t98 = ((t10).sqrt());
            let t99 = t98 * t1;
            let t100 = t99 * t93;
            let t105 = t20 * t5 / t21 / v_rho;
            let t107 = -f64x8::splat(0.632975) * t94 - f64x8::splat(0.29896666666666666) * t96 - f64x8::splat(0.1023875) * t100 - f64x8::splat(0.08215666666666667) * t105;
            let t108 = f64x8::splat(1.0) / t29;
            let t109 = t107 * t108;
            let t112 = t43 * t1;
            let t117 = t43 * t45;
            let t118 = t50 * t50;
            let t119 = f64x8::splat(1.0) / t118;
            let t124 = -f64x8::splat(0.8630833333333333) * t94 - f64x8::splat(0.301925) * t96 - f64x8::splat(0.05501625) * t100 - f64x8::splat(0.082785) * t105;
            let t126 = f64x8::splat(1.0) / t53;
            let t127 = t119 * t124 * t126;
            let t130 = f64x8::splat(0.0011073470983333333) * t4 * t83 * t30 + f64x8::splat(1.0) * t89 * t109 - f64x8::splat(0.00018311447306006544) * t112 * t92 * t82 * t54 - f64x8::splat(0.5848223622634646) * t117 * t127;
            let t131 = v_rho * t130;
            let t132 = t59 * t80;
            let t134 = v_rho * t58;
            let t135 = t76 * t76;
            let t136 = f64x8::splat(1.0) / t135;
            let t138 = t136 * param_alpha * t61;
            let t139 = t66 * v_sigma;
            let t140 = t70 * v_rho;
            let t142 = f64x8::splat(1.0) / t21 / t140;
            let t143 = t68 * t142;
            let t144 = t139 * t143;
            let t146 = param_gamma * t136;
            let t147 = t146 * t62;
            let t150 = t138 * t144 / f64x8::splat(9.0) - t147 * t144 / f64x8::splat(9.0);
            let t151 = t59 * t150;
            let tvrho0 = t131 * t132 + t134 * t151 + tzk0;
            acc_vrho = tvrho0;
            let t153 = t66 * t68;
            let t156 = t146 * param_alpha;
            let t157 = t61 * t66;
            let t162 = t156 * t157 * t68 * t72 / f64x8::splat(24.0) - t138 * t153 * t72 / f64x8::splat(24.0);
            let t163 = t59 * t162;
            let tvsigma0 = t134 * t163;
            acc_vsigma = tvsigma0;
            let t164 = t130 * t59;
            let t170 = f64x8::splat(1.0) / t7 / t70;
            let t171 = t6 * t170;
            let t175 = t4 * t6;
            let t176 = t82 * t88;
            let t180 = t87 * t26;
            let t181 = f64x8::splat(1.0) / t180;
            let t182 = t12 * t181;
            let t183 = t107 * t107;
            let t184 = t183 * t108;
            let t189 = f64x8::splat(1.0) / t13 / t10 * t18;
            let t190 = t19 * t5;
            let t191 = t190 * t72;
            let t192 = t189 * t191;
            let t194 = t92 * t170;
            let t195 = t91 * t194;
            let t197 = t4 * t171;
            let t199 = f64x8::splat(1.0)/((t10).sqrt());
            let t200 = t199 * t18;
            let t201 = t200 * t191;
            let t203 = t99 * t194;
            let t206 = t20 * t5 * t72;
            let t208 = -f64x8::splat(0.4219833333333333) * t192 + f64x8::splat(0.8439666666666666) * t195 + f64x8::splat(0.3986222222222222) * t197 + f64x8::splat(0.06825833333333334) * t201 + f64x8::splat(0.13651666666666668) * t203 + f64x8::splat(0.1369277777777778) * t206;
            let t209 = t208 * t108;
            let t212 = t87 * t87;
            let t213 = f64x8::splat(1.0) / t212;
            let t214 = t12 * t213;
            let t215 = t29 * t29;
            let t216 = f64x8::splat(1.0) / t215;
            let t217 = t183 * t216;
            let t224 = t43 * t4;
            let t228 = t118 * t50;
            let t229 = f64x8::splat(1.0) / t228;
            let t230 = t124 * t124;
            let t232 = t229 * t230 * t126;
            let t241 = -f64x8::splat(0.5753888888888888) * t192 + f64x8::splat(1.1507777777777777) * t195 + f64x8::splat(0.4025666666666667) * t197 + f64x8::splat(0.0366775) * t201 + f64x8::splat(0.073355) * t203 + f64x8::splat(0.137975) * t206;
            let t243 = t119 * t241 * t126;
            let t246 = t118 * t118;
            let t247 = f64x8::splat(1.0) / t246;
            let t248 = t247 * t230;
            let t249 = t53 * t53;
            let t250 = f64x8::splat(1.0) / t249;
            let t251 = t248 * t250;
            let t254 = -f64x8::splat(0.0014764627977777779) * t4 * t171 * t30 - f64x8::splat(0.035616666666666665) * t175 * t176 * t109 - f64x8::splat(2.0) * t182 * t184 + f64x8::splat(1.0) * t89 * t209 + f64x8::splat(16.081979498692537) * t214 * t217 + f64x8::splat(0.00024415263074675396) * t112 * t92 * t170 * t54 + f64x8::splat(0.01084358130030174) * t224 * t83 * t127 + f64x8::splat(1.1696447245269292) * t117 * t232 - f64x8::splat(0.5848223622634646) * t117 * t243 - f64x8::splat(17.315859105681465) * t117 * t251;
            let t255 = v_rho * t254;
            let t260 = f64x8::splat(1.0) / t135 / t76;
            let t261 = param_alpha * param_alpha;
            let t263 = t61 * t61;
            let t264 = t260 * t261 * t263;
            let t266 = f64x8::splat(1.0) / t64 / t63;
            let t267 = v_sigma * v_sigma;
            let t268 = t266 * t267;
            let t269 = t70 * t70;
            let t272 = f64x8::splat(1.0) / t7 / t269 / t140;
            let t273 = t39 * t272;
            let t274 = t268 * t273;
            let t278 = f64x8::splat(1.0) / t21 / t269;
            let t279 = t68 * t278;
            let t280 = t139 * t279;
            let t283 = param_gamma * t260;
            let t285 = t283 * t261 * t263;
            let t290 = f64x8::splat(4.0) / f64x8::splat(81.0) * t264 * t274 - f64x8::splat(11.0) / f64x8::splat(27.0) * t138 * t280 - f64x8::splat(4.0) / f64x8::splat(81.0) * t285 * t274 + f64x8::splat(11.0) / f64x8::splat(27.0) * t147 * t280;
            let t291 = t59 * t290;
            let tv2rho20 = f64x8::splat(2.0) * t131 * t151 + t255 * t132 + t134 * t291 + f64x8::splat(2.0) * t60 * t150 + f64x8::splat(2.0) * t164 * t80;
            acc_v2rho2 = tv2rho20;
            let t295 = t266 * t39;
            let t296 = t269 * t70;
            let t298 = f64x8::splat(1.0) / t7 / t296;
            let t300 = t295 * t298 * v_sigma;
            let t311 = -t264 * t300 / f64x8::splat(54.0) + t138 * t153 * t142 / f64x8::splat(9.0) + t285 * t300 / f64x8::splat(54.0) - t156 * t157 * t143 / f64x8::splat(9.0);
            let t312 = t59 * t311;
            let tv2rhosigma0 = t131 * t163 + t134 * t312 + t60 * t162;
            acc_v2rhosigma = tv2rhosigma0;
            let t314 = t269 * v_rho;
            let t316 = f64x8::splat(1.0) / t7 / t314;
            let t319 = t283 * t261;
            let t320 = t263 * t266;
            let t325 = -t319 * t320 * t39 * t316 / f64x8::splat(144.0) + t264 * t295 * t316 / f64x8::splat(144.0);
            let t326 = t59 * t325;
            let tv2sigma20 = t134 * t326;
            acc_v2sigma2 = tv2sigma20;
            let t327 = t254 * t59;
            let t347 = f64x8::splat(1.0) / t212 / t26;
            let t348 = t12 * t347;
            let t349 = t183 * t107;
            let t350 = t349 * t216;
            let t356 = f64x8::splat(1.0) / t13 / t24 * t2 / f64x8::splat(4.0);
            let t357 = f64x8::splat(1.0) / t269;
            let t358 = t356 * t357;
            let t360 = t190 * t142;
            let t361 = t189 * t360;
            let t364 = f64x8::splat(1.0) / t7 / t140;
            let t365 = t92 * t364;
            let t366 = t91 * t365;
            let t368 = t6 * t364;
            let t369 = t4 * t368;
            let t371 = f64x8::splat(1.0)/((t10) * (t10).sqrt());
            let t372 = t371 * t2;
            let t373 = t372 * t357;
            let t375 = t200 * t360;
            let t377 = t99 * t365;
            let t380 = t20 * t5 * t142;
            let t382 = -f64x8::splat(2.5319) * t358 + f64x8::splat(1.6879333333333333) * t361 - f64x8::splat(1.9692555555555555) * t366 - f64x8::splat(0.9301185185185186) * t369 + f64x8::splat(0.13651666666666668) * t373 - f64x8::splat(0.27303333333333335) * t375 - f64x8::splat(0.31853888888888887) * t377 - f64x8::splat(0.36514074074074077) * t380;
            let t383 = t382 * t108;
            let t387 = f64x8::splat(1.0) / t212 / t87;
            let t388 = t12 * t387;
            let t390 = f64x8::splat(1.0) / t215 / t29;
            let t391 = t349 * t390;
            let t402 = t349 * t108;
            let t408 = -f64x8::splat(0.03253074390090522) * t224 * t83 * t232 - f64x8::splat(0.02168716260060348) * t224 * t171 * t127 + f64x8::splat(0.01626537195045261) * t224 * t83 * t243 + f64x8::splat(0.4815973313767657) * t224 * t83 * t251 - f64x8::splat(96.49187699215521) * t348 * t350 + f64x8::splat(1.0) * t89 * t383 + f64x8::splat(517.260129192734) * t388 * t391 + f64x8::splat(0.10685) * t175 * t82 * t181 * t184 - f64x8::splat(0.0005696894717424259) * t112 * t92 * t364 * t54 + f64x8::splat(6.0) * t214 * t402 - f64x8::splat(6.0) * t182 * t109 * t208;
            let t414 = f64x8::splat(1.0) / t246 / t50;
            let t415 = t230 * t124;
            let t417 = t414 * t415 * t250;
            let t428 = -f64x8::splat(3.4523333333333333) * t358 + f64x8::splat(2.3015555555555554) * t361 - f64x8::splat(2.6851481481481483) * t366 - f64x8::splat(0.9393222222222222) * t369 + f64x8::splat(0.073355) * t373 - f64x8::splat(0.14671) * t375 - f64x8::splat(0.17116166666666666) * t377 - f64x8::splat(0.36793333333333333) * t380;
            let t430 = t119 * t428 * t126;
            let t434 = f64x8::splat(1.0) / t246 / t118;
            let t437 = f64x8::splat(1.0) / t249 / t53;
            let t438 = t434 * t415 * t437;
            let t444 = t170 * t88;
            let t451 = t82 * t213;
            let t456 = t247 * t415 * t126;
            let t460 = t126 * t241;
            let t465 = t250 * t124;
            let t469 = f64x8::splat(48.245938496077606) * t214 * t208 * t216 * t107 + f64x8::splat(103.89515463408878) * t117 * t417 - f64x8::splat(0.5848223622634646) * t117 * t430 - f64x8::splat(1025.4018858216407) * t117 * t438 + f64x8::splat(0.0034450798614814814) * t4 * t368 * t30 + f64x8::splat(0.07123333333333333) * t175 * t444 * t109 - f64x8::splat(0.053425) * t175 * t176 * t209 - f64x8::splat(0.8591797547176487) * t175 * t451 * t217 - f64x8::splat(3.5089341735807875) * t117 * t456 + f64x8::splat(3.5089341735807875) * t117 * t229 * t124 * t460 - f64x8::splat(51.94757731704439) * t117 * t247 * t241 * t465;
            let t470 = t408 + t469;
            let t471 = v_rho * t470;
            let t477 = t135 * t135;
            let t478 = f64x8::splat(1.0) / t477;
            let t479 = t261 * param_alpha;
            let t480 = t478 * t479;
            let t481 = t63 * t63;
            let t482 = f64x8::splat(1.0) / t481;
            let t483 = t267 * v_sigma;
            let t484 = t482 * t483;
            let t485 = t269 * t269;
            let t486 = t485 * t140;
            let t487 = f64x8::splat(1.0) / t486;
            let t488 = t484 * t487;
            let t492 = f64x8::splat(1.0) / t7 / t485;
            let t494 = t268 * t39 * t492;
            let t498 = f64x8::splat(1.0) / t21 / t314;
            let t499 = t68 * t498;
            let t500 = t139 * t499;
            let t503 = param_gamma * t478;
            let t504 = t503 * t479;
            let t511 = f64x8::splat(16.0) / f64x8::splat(81.0) * t480 * t488 - f64x8::splat(44.0) / f64x8::splat(81.0) * t264 * t494 + f64x8::splat(154.0) / f64x8::splat(81.0) * t138 * t500 - f64x8::splat(16.0) / f64x8::splat(81.0) * t504 * t488 + f64x8::splat(44.0) / f64x8::splat(81.0) * t285 * t494 - f64x8::splat(154.0) / f64x8::splat(81.0) * t147 * t500;
            let t512 = t59 * t511;
            let tv3rho30 = f64x8::splat(3.0) * t131 * t291 + t471 * t132 + t134 * t512 + f64x8::splat(6.0) * t164 * t150 + f64x8::splat(3.0) * t255 * t151 + f64x8::splat(3.0) * t60 * t290 + f64x8::splat(3.0) * t327 * t80;
            acc_v3rho3 = tv3rho30;
            let t521 = t485 * t70;
            let t523 = t482 / t521;
            let t524 = t523 * t267;
            let t528 = t295 * t272 * v_sigma;
            let t541 = -f64x8::splat(2.0) / f64x8::splat(27.0) * t480 * t524 + t264 * t528 / f64x8::splat(6.0) - f64x8::splat(11.0) / f64x8::splat(27.0) * t138 * t153 * t278 + f64x8::splat(2.0) / f64x8::splat(27.0) * t504 * t524 - t285 * t528 / f64x8::splat(6.0) + f64x8::splat(11.0) / f64x8::splat(27.0) * t156 * t157 * t279;
            let t542 = t59 * t541;
            let tv3rho2sigma0 = f64x8::splat(2.0) * t131 * t312 + t134 * t542 + f64x8::splat(2.0) * t164 * t162 + t255 * t163 + f64x8::splat(2.0) * t60 * t311;
            acc_v3rho2sigma = tv3rho2sigma0;
            let t546 = t485 * v_rho;
            let t547 = f64x8::splat(1.0) / t546;
            let t548 = t482 * t547;
            let t549 = t548 * v_sigma;
            let t561 = t480 * t549 / f64x8::splat(36.0) - t264 * t295 * t298 / f64x8::splat(27.0) - t504 * t549 / f64x8::splat(36.0) + t319 * t320 * t39 * t298 / f64x8::splat(27.0);
            let t562 = t59 * t561;
            let tv3rhosigma20 = t131 * t326 + t134 * t562 + t60 * t325;
            acc_v3rhosigma2 = tv3rhosigma20;
            let t564 = f64x8::splat(1.0) / t485;
            let t567 = t479 * t482;
            let t571 = -t480 * t482 * t564 / f64x8::splat(96.0) + t503 * t567 * t564 / f64x8::splat(96.0);
            let t572 = t59 * t571;
            let tv3sigma30 = t134 * t572;
            acc_v3sigma3 = tv3sigma30;
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
        ip += 8;
    }
}
