//! GGA_X_HTBS fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_htbs.c`
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
pub fn gga_x_htbs_fxc_unpol(
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
            let t22 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t23 = (simd::cbrt(t22));
            let t25 = t21 / t23;
            let t26 = ((v_sigma).sqrt());
            let t27 = f64x8::splat(M_CBRT2);
            let t28 = t26 * t27;
            let t30 = f64x8::splat(1.0) / t18 / v_rho;
            let t32 = t25 * t28 * t30;
            let t33 = t32 / f64x8::splat(12.0);
            let t34 = (t33).simd_le(f64x8::splat(0.6));
            let t35 = t23 * t23;
            let t36 = f64x8::splat(1.0) / t35;
            let t37 = t20 * t36;
            let t38 = t27 * t27;
            let t39 = v_sigma * t38;
            let t40 = v_rho * v_rho;
            let t41 = t18 * t18;
            let t43 = f64x8::splat(1.0) / t41 / t40;
            let t45 = t37 * t39 * t43;
            let t47 = t37 * v_sigma;
            let t48 = t38 * t43;
            let t50 = (simd::exp(-t45 / f64x8::splat(24.0)));
            let t51 = t48 * t50;
            let t55 = f64x8::splat(1.0) / t23 / t22;
            let t56 = t21 * t55;
            let t57 = v_sigma * v_sigma;
            let t58 = t57 * t27;
            let t59 = t40 * t40;
            let t60 = t59 * v_rho;
            let t62 = f64x8::splat(1.0) / t18 / t60;
            let t64 = t56 * t58 * t62;
            let t66 = f64x8::splat(1.0) + f64x8::splat(2.7560657413756314e-05) * t64;
            let t67 = (simd::ln(t66));
            let t68 = f64x8::splat(0.804) + f64x8::splat(5.0) / f64x8::splat(972.0) * t45 + f64x8::splat(0.004002424276710846) * t47 * t51 + t67;
            let t71 = f64x8::splat(1.804) - f64x8::splat(0.646416) / t68;
            let t72 = (f64x8::splat(2.6)).simd_le(t33);
            let t74 = (simd::exp(-f64x8::splat(0.011376190545424806) * t45));
            let t76 = f64x8::splat(1.804) - f64x8::splat(0.804) * t74;
            let t77 = f64x8::splat(0.190125) * t32;
            let t78 = f64x8::splat(0.195) * t45;
            let t79 = t26 * v_sigma;
            let t80 = f64x8::splat(1.0) / t59;
            let t82 = f64x8::splat(0.017625664237781676) * t79 * t80;
            let t83 = f64x8::splat(0.005208333333333333) * t64;
            let t86 = t20 / t35 / t22;
            let t87 = t26 * t57;
            let t88 = t87 * t38;
            let t89 = t59 * t40;
            let t91 = f64x8::splat(1.0) / t41 / t89;
            let t94 = f64x8::splat(0.0003255208333333333) * t86 * t88 * t91;
            let t95 = -f64x8::splat(0.40608) + t77 - t78 + t82 - t83 + t94;
            let t97 = f64x8::splat(1.40608) - t77 + t78 - t82 + t83 - t94;
            let t100 = ((t34).select(t71, (t72).select(t76, t97 * t71 + t95 * t76)));
            let t104 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t100));
            let tzk0 = f64x8::splat(2.0) * t104;
            acc_zk = tzk0;
            let t106 = t17 / t41;
            let t110 = t68 * t68;
            let t111 = f64x8::splat(1.0) / t110;
            let t112 = t40 * v_rho;
            let t114 = f64x8::splat(1.0) / t41 / t112;
            let t116 = t37 * t39 * t114;
            let t118 = t38 * t114;
            let t119 = t118 * t50;
            let t122 = t56 * t57;
            let t124 = f64x8::splat(1.0) / t18 / t89;
            let t125 = t27 * t124;
            let t126 = t125 * t50;
            let t129 = f64x8::splat(1.0) / t66;
            let t130 = t125 * t129;
            let t133 = -f64x8::splat(10.0) / f64x8::splat(729.0) * t116 - f64x8::splat(0.010673131404562256) * t47 * t119 + f64x8::splat(0.0008894276170468547) * t122 * t126 - f64x8::splat(0.00014699017287336702) * t122 * t130;
            let t136 = t118 * t74;
            let t140 = f64x8::splat(1.0) / t18 / t40;
            let t145 = f64x8::splat(1.0) / t60;
            let t151 = t59 * t112;
            let t153 = f64x8::splat(1.0) / t41 / t151;
            let t157 = -f64x8::splat(0.2535) * t25 * t28 * t140 + f64x8::splat(0.52) * t116 - f64x8::splat(0.0705026569511267) * t79 * t145 + f64x8::splat(0.027777777777777776) * t56 * t58 * t124 - f64x8::splat(0.002170138888888889) * t86 * t88 * t153;
            let t160 = t95 * t20 * t36;
            let t162 = t39 * t114 * t74;
            let t165 = -t157;
            let t167 = t97 * t111;
            let t171 = ((t34).select(f64x8::splat(0.646416) * t111 * t133, (t72).select(-f64x8::splat(0.024390552529390784) * t47 * t136, t157 * t76 - f64x8::splat(0.024390552529390784) * t160 * t162 + t165 * t71 + f64x8::splat(0.646416) * t167 * t133)));
            let t176 = ((t2).select(f64x8::splat(0.0), -t6 * t106 * t100 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t171));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t176 + f64x8::splat(2.0) * t104;
            acc_vrho = tvrho0;
            let t179 = t37 * t48;
            let t183 = t56 * v_sigma;
            let t184 = t27 * t62;
            let t185 = t184 * t50;
            let t188 = t184 * t129;
            let t191 = f64x8::splat(5.0) / f64x8::splat(972.0) * t179 + f64x8::splat(0.004002424276710846) * t37 * t51 - f64x8::splat(0.0003335353563925705) * t183 * t185 + f64x8::splat(5.512131482751263e-05) * t183 * t188;
            let t194 = t48 * t74;
            let t197 = f64x8::splat(1.0) / t26;
            let t198 = t197 * t27;
            let t205 = v_sigma * t27;
            let t209 = t79 * t38;
            let t213 = f64x8::splat(0.0950625) * t25 * t198 * t30 - f64x8::splat(0.195) * t179 + f64x8::splat(0.026438496356672513) * t26 * t80 - f64x8::splat(0.010416666666666666) * t56 * t205 * t62 + f64x8::splat(0.0008138020833333334) * t86 * t209 * t91;
            let t217 = -t213;
            let t222 = ((t34).select(f64x8::splat(0.646416) * t111 * t191, (t72).select(f64x8::splat(0.009146457198521543) * t37 * t194, t213 * t76 + f64x8::splat(0.009146457198521543) * t160 * t194 + t217 * t71 + f64x8::splat(0.646416) * t167 * t191)));
            let t226 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t222));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t226;
            acc_vsigma = tvsigma0;
            let t231 = t17 / t41 / v_rho;
            let t239 = f64x8::splat(1.0) / t110 / t68;
            let t240 = t133 * t133;
            let t244 = f64x8::splat(1.0) / t41 / t59;
            let t246 = t37 * t39 * t244;
            let t248 = t38 * t244;
            let t249 = t248 * t50;
            let t253 = f64x8::splat(1.0) / t18 / t151;
            let t254 = t27 * t253;
            let t255 = t254 * t50;
            let t258 = t22 * t22;
            let t259 = f64x8::splat(1.0) / t258;
            let t260 = t57 * v_sigma;
            let t261 = t259 * t260;
            let t262 = t59 * t59;
            let t263 = t262 * t40;
            let t264 = f64x8::splat(1.0) / t263;
            let t268 = t254 * t129;
            let t273 = t20 / t35 / t258;
            let t274 = t57 * t57;
            let t275 = t273 * t274;
            let t276 = t262 * t59;
            let t278 = f64x8::splat(1.0) / t41 / t276;
            let t280 = t66 * t66;
            let t281 = f64x8::splat(1.0) / t280;
            let t282 = t38 * t278 * t281;
            let t285 = f64x8::splat(110.0) / f64x8::splat(2187.0) * t246 + f64x8::splat(0.039134815150061605) * t47 * t249 - f64x8::splat(0.008004848553421692) * t122 * t255 + f64x8::splat(0.0011859034893958063) * t261 * t264 * t50 + f64x8::splat(0.0009309377615313244) * t122 * t268 - f64x8::splat(1.2963666552805393e-07) * t275 * t282;
            let t289 = t248 * t74;
            let t292 = t254 * t74;
            let t297 = f64x8::splat(1.0) / t18 / t112;
            let t302 = f64x8::splat(1.0) / t89;
            let t309 = f64x8::splat(1.0) / t41 / t262;
            let t313 = f64x8::splat(0.5915) * t25 * t28 * t297 - f64x8::splat(1.9066666666666667) * t246 + f64x8::splat(0.3525132847556335) * t79 * t302 - f64x8::splat(0.17592592592592593) * t56 * t58 * t253 + f64x8::splat(0.016637731481481483) * t86 * t88 * t309;
            let t316 = t157 * t20 * t36;
            let t320 = t39 * t244 * t74;
            let t324 = t95 * t21 * t55;
            let t326 = t58 * t253 * t74;
            let t329 = -t313;
            let t331 = t165 * t111;
            let t334 = t97 * t239;
            let t340 = ((t34).select(-f64x8::splat(1.292832) * t239 * t240 + f64x8::splat(0.646416) * t111 * t285, (t72).select(f64x8::splat(0.08943202594109954) * t47 * t289 - f64x8::splat(0.0014798483897735602) * t122 * t292, t313 * t76 - f64x8::splat(0.04878110505878157) * t316 * t162 + f64x8::splat(0.08943202594109954) * t160 * t320 - f64x8::splat(0.0014798483897735602) * t324 * t326 + t329 * t71 + f64x8::splat(1.292832) * t331 * t133 - f64x8::splat(1.292832) * t334 * t240 + f64x8::splat(0.646416) * t167 * t285)));
            let t345 = ((t2).select(f64x8::splat(0.0), t6 * t231 * t100 / f64x8::splat(12.0) - t6 * t106 * t171 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t340));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t345 + f64x8::splat(4.0) * t176;
            acc_v2rho2 = tv2rho20;
            let t351 = t239 * t191;
            let t354 = t37 * t118;
            let t358 = t56 * t27;
            let t359 = t124 * v_sigma;
            let t363 = t259 * t57;
            let t364 = t262 * v_rho;
            let t365 = f64x8::splat(1.0) / t364;
            let t371 = t273 * t260;
            let t372 = t262 * t112;
            let t374 = f64x8::splat(1.0) / t41 / t372;
            let t379 = -f64x8::splat(10.0) / f64x8::splat(729.0) * t354 - f64x8::splat(0.010673131404562256) * t37 * t119 + f64x8::splat(0.002668282851140564) * t358 * t359 * t50 - f64x8::splat(0.00044471380852342736) * t363 * t365 * t50 - f64x8::splat(0.00029398034574673403) * t183 * t130 + f64x8::splat(4.861374957302022e-08) * t371 * t38 * t374 * t281;
            let t401 = -f64x8::splat(0.12675) * t25 * t198 * t140 + f64x8::splat(0.52) * t354 - f64x8::splat(0.10575398542669005) * t26 * t145 + f64x8::splat(0.05555555555555555) * t56 * t205 * t124 - f64x8::splat(0.005425347222222222) * t86 * t209 * t153;
            let t404 = t213 * t20 * t36;
            let t411 = v_sigma * t74;
            let t412 = t125 * t411;
            let t415 = -t401;
            let t417 = t217 * t111;
            let t422 = t191 * t133;
            let t428 = ((t34).select(-f64x8::splat(1.292832) * t351 * t133 + f64x8::splat(0.646416) * t111 * t379, (t72).select(-f64x8::splat(0.024390552529390784) * t37 * t136 + f64x8::splat(0.000554943146165085) * t358 * t359 * t74, t401 * t76 - f64x8::splat(0.024390552529390784) * t404 * t162 + f64x8::splat(0.009146457198521543) * t316 * t194 - f64x8::splat(0.024390552529390784) * t160 * t136 + f64x8::splat(0.000554943146165085) * t324 * t412 + t415 * t71 + f64x8::splat(0.646416) * t417 * t133 + f64x8::splat(0.646416) * t331 * t191 - f64x8::splat(1.292832) * t334 * t422 + f64x8::splat(0.646416) * t167 * t379)));
            let t433 = ((t2).select(f64x8::splat(0.0), -t6 * t106 * t222 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t428));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t433 + f64x8::splat(2.0) * t226;
            acc_v2rhosigma = tv2rhosigma0;
            let t436 = t191 * t191;
            let t441 = t259 * v_sigma;
            let t442 = f64x8::splat(1.0) / t262;
            let t450 = f64x8::splat(1.0) / t41 / t263;
            let t452 = t38 * t450 * t281;
            let t455 = -f64x8::splat(0.000667070712785141) * t56 * t185 + f64x8::splat(0.00016676767819628525) * t441 * t442 * t50 + f64x8::splat(5.512131482751263e-05) * t56 * t188 - f64x8::splat(1.8230156089882582e-08) * t273 * t57 * t452;
            let t459 = t184 * t74;
            let t462 = f64x8::splat(1.0) / t79;
            let t463 = t462 * t27;
            let t471 = t26 * t38;
            let t475 = -f64x8::splat(0.04753125) * t25 * t463 * t30 + f64x8::splat(0.013219248178336257) * t197 * t80 - f64x8::splat(0.010416666666666666) * t56 * t184 + f64x8::splat(0.001220703125) * t86 * t471 * t91;
            let t481 = -t475;
            let t490 = ((t34).select(-f64x8::splat(1.292832) * t239 * t436 + f64x8::splat(0.646416) * t111 * t455, (t72).select(-f64x8::splat(0.0002081036798119069) * t56 * t459, t475 * t76 + f64x8::splat(0.018292914397043086) * t404 * t194 - f64x8::splat(0.0002081036798119069) * t324 * t459 + t481 * t71 + f64x8::splat(1.292832) * t417 * t191 - f64x8::splat(1.292832) * t334 * t436 + f64x8::splat(0.646416) * t167 * t455)));
            let t494 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t490));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t494;
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
