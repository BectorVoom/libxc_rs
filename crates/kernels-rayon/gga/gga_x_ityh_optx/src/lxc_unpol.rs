//! GGA_X_ITYH_OPTX lxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_ityh_optx.c`
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
pub fn gga_x_ityh_optx_lxc_unpol(
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
    param_b: f64,
    param_a: f64,
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_b = f64x8::splat(param_b);
    let param_a = f64x8::splat(param_a);
    let param_hyb_omega_0 = f64x8::splat(param_hyb_omega_0);
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
            let t7 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t8 = zeta_threshold - f64x8::splat(1.0);
            let t10 = ((t7).select(t8, (t7).select(-t8, f64x8::splat(0.0))));
            let t11 = f64x8::splat(1.0) + t10;
            let t13 = (simd::cbrt(zeta_threshold));
            let t15 = (simd::cbrt(t11));
            let t17 = (((t11).simd_le(zeta_threshold)).select(t13 * zeta_threshold, t15 * t11));
            let t18 = t3 / t4 * t17;
            let t19 = (simd::cbrt(v_rho));
            let t20 = t3 * t3;
            let t22 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t23 = (simd::cbrt(t22));
            let t24 = f64x8::splat(1.0) / t23;
            let t25 = f64x8::splat(M_CBRT4);
            let t26 = t24 * t25;
            let t27 = v_sigma * v_sigma;
            let t28 = param_b * t27;
            let t29 = f64x8::splat(M_CBRT2);
            let t30 = v_rho * v_rho;
            let t31 = t30 * t30;
            let t32 = t31 * v_rho;
            let t34 = f64x8::splat(1.0) / t19 / t32;
            let t36 = t29 * t29;
            let t38 = t19 * t19;
            let t40 = f64x8::splat(1.0) / t38 / t30;
            let t43 = f64x8::splat(1.0) + f64x8::splat(6.0) * v_sigma * t36 * t40;
            let t44 = t43 * t43;
            let t45 = f64x8::splat(1.0) / t44;
            let t46 = t29 * t34 * t45;
            let t49 = param_a + f64x8::splat(72.0) * t28 * t46;
            let t52 = f64x8::splat(M_PI) * t20 * t26 / t49;
            let t53 = ((t52).sqrt());
            let t55 = param_hyb_omega_0 / t53;
            let t56 = t11 * v_rho;
            let t57 = (simd::cbrt(t56));
            let t58 = f64x8::splat(1.0) / t57;
            let t59 = t29 * t58;
            let t61 = t55 * t59 / f64x8::splat(2.0);
            let t62 = (f64x8::splat(1.35)).simd_le(t61);
            let t63 = (f64x8::splat(1.35)).simd_lt(t61);
            let t64 = ((t63).select(t61, f64x8::splat(1.35)));
            let t65 = t64 * t64;
            let t68 = t65 * t65;
            let t69 = f64x8::splat(1.0) / t68;
            let t71 = t68 * t65;
            let t72 = f64x8::splat(1.0) / t71;
            let t74 = t68 * t68;
            let t75 = f64x8::splat(1.0) / t74;
            let t78 = f64x8::splat(1.0) / t74 / t65;
            let t81 = f64x8::splat(1.0) / t74 / t68;
            let t84 = f64x8::splat(1.0) / t74 / t71;
            let t86 = t74 * t74;
            let t87 = f64x8::splat(1.0) / t86;
            let t90 = ((t63).select(f64x8::splat(1.35), t61));
            let t91 = ((f64x8::splat(M_PI)).sqrt());
            let t92 = f64x8::splat(1.0) / t90;
            let t94 = (simd::erf(t92 / f64x8::splat(2.0)));
            let t96 = t90 * t90;
            let t97 = f64x8::splat(1.0) / t96;
            let t99 = (simd::exp(-t97 / f64x8::splat(4.0)));
            let t100 = t99 - f64x8::splat(1.0);
            let t103 = t99 - f64x8::splat(3.0) / f64x8::splat(2.0) - f64x8::splat(2.0) * t96 * t100;
            let t106 = f64x8::splat(2.0) * t90 * t103 + t91 * t94;
            let t110 = ((t62).select(f64x8::splat(1.0) / t65 / f64x8::splat(36.0) - t69 / f64x8::splat(960.0) + t72 / f64x8::splat(26880.0) - t75 / f64x8::splat(829440.0) + t78 / f64x8::splat(28385280.0) - t81 / f64x8::splat(1073479680.0) + t84 / f64x8::splat(44590694400.0) - t87 / f64x8::splat(2021444812800.0), f64x8::splat(1.0) - f64x8::splat(8.0) / f64x8::splat(3.0) * t90 * t106));
            let t111 = t19 * t110;
            let t115 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t111 * t49));
            let tzk0 = f64x8::splat(2.0) * t115;
            acc_zk = tzk0;
            let t116 = f64x8::splat(1.0) / t38;
            let t117 = t116 * t110;
            let t121 = t65 * t64;
            let t122 = f64x8::splat(1.0) / t121;
            let t125 = param_hyb_omega_0 / t53 / t52;
            let t127 = t125 * t59 * f64x8::splat(M_PI);
            let t128 = t20 * t24;
            let t129 = t49 * t49;
            let t130 = f64x8::splat(1.0) / t129;
            let t131 = t25 * t130;
            let t132 = t31 * t30;
            let t134 = f64x8::splat(1.0) / t19 / t132;
            let t136 = t29 * t134 * t45;
            let t140 = param_b * t27 * v_sigma;
            let t141 = t31 * t31;
            let t142 = t141 * v_rho;
            let t143 = f64x8::splat(1.0) / t142;
            let t145 = f64x8::splat(1.0) / t44 / t43;
            let t146 = t143 * t145;
            let t149 = -f64x8::splat(384.0) * t28 * t136 + f64x8::splat(4608.0) * t140 * t146;
            let t155 = f64x8::splat(1.0) / t57 / t56;
            let t156 = t29 * t155;
            let t160 = t127 * t128 * t131 * t149 / f64x8::splat(4.0) - t55 * t156 * t11 / f64x8::splat(6.0);
            let t161 = ((t63).select(t160, f64x8::splat(0.0)));
            let t164 = t68 * t64;
            let t165 = f64x8::splat(1.0) / t164;
            let t168 = t68 * t121;
            let t169 = f64x8::splat(1.0) / t168;
            let t173 = f64x8::splat(1.0) / t74 / t64;
            let t177 = f64x8::splat(1.0) / t74 / t121;
            let t181 = f64x8::splat(1.0) / t74 / t164;
            let t185 = f64x8::splat(1.0) / t74 / t168;
            let t189 = f64x8::splat(1.0) / t86 / t64;
            let t193 = ((t63).select(f64x8::splat(0.0), t160));
            let t195 = t99 * t97;
            let t199 = t96 * t90;
            let t200 = f64x8::splat(1.0) / t199;
            let t204 = t90 * t100;
            let t209 = t200 * t193 * t99 / f64x8::splat(2.0) - f64x8::splat(4.0) * t204 * t193 - t92 * t193 * t99;
            let t212 = f64x8::splat(2.0) * t193 * t103 - t195 * t193 + f64x8::splat(2.0) * t90 * t209;
            let t216 = ((t62).select(-t122 * t161 / f64x8::splat(18.0) + t165 * t161 / f64x8::splat(240.0) - t169 * t161 / f64x8::splat(4480.0) + t173 * t161 / f64x8::splat(103680.0) - t177 * t161 / f64x8::splat(2838528.0) + t181 * t161 / f64x8::splat(89456640.0) - t185 * t161 / f64x8::splat(3185049600.0) + t189 * t161 / f64x8::splat(126340300800.0), -f64x8::splat(8.0) / f64x8::splat(3.0) * t193 * t106 - f64x8::splat(8.0) / f64x8::splat(3.0) * t90 * t212));
            let t217 = t19 * t216;
            let t225 = ((t2).select(f64x8::splat(0.0), -t18 * t117 * t49 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t217 * t49 - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t111 * t149));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t225 + f64x8::splat(2.0) * t115;
            acc_vrho = tvrho0;
            let t228 = param_b * v_sigma;
            let t231 = f64x8::splat(1.0) / t141;
            let t232 = t231 * t145;
            let t235 = f64x8::splat(144.0) * t228 * t46 - f64x8::splat(1728.0) * t28 * t232;
            let t239 = t127 * t128 * t131 * t235 / f64x8::splat(4.0);
            let t240 = ((t63).select(t239, f64x8::splat(0.0)));
            let t243 = t165 * t240;
            let t245 = t169 * t240;
            let t247 = t173 * t240;
            let t249 = t177 * t240;
            let t251 = t181 * t240;
            let t253 = t185 * t240;
            let t255 = t189 * t240;
            let t258 = ((t63).select(f64x8::splat(0.0), t239));
            let t270 = t200 * t258 * t99 / f64x8::splat(2.0) - f64x8::splat(4.0) * t204 * t258 - t92 * t258 * t99;
            let t273 = f64x8::splat(2.0) * t258 * t103 - t195 * t258 + f64x8::splat(2.0) * t90 * t270;
            let t277 = ((t62).select(-t122 * t240 / f64x8::splat(18.0) + t243 / f64x8::splat(240.0) - t245 / f64x8::splat(4480.0) + t247 / f64x8::splat(103680.0) - t249 / f64x8::splat(2838528.0) + t251 / f64x8::splat(89456640.0) - t253 / f64x8::splat(3185049600.0) + t255 / f64x8::splat(126340300800.0), -f64x8::splat(8.0) / f64x8::splat(3.0) * t258 * t106 - f64x8::splat(8.0) / f64x8::splat(3.0) * t90 * t273));
            let t278 = t19 * t277;
            let t285 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t111 * t235 - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t278 * t49));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t285;
            acc_vsigma = tvsigma0;
            let t289 = f64x8::splat(1.0) / t38 / v_rho;
            let t290 = t289 * t110;
            let t294 = t116 * t216;
            let t301 = t161 * t161;
            let t304 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t306 = t23 * t23;
            let t307 = f64x8::splat(1.0) / t306;
            let t308 = t25 * t25;
            let t309 = t307 * t308;
            let t315 = param_hyb_omega_0 / t53 / t304 / t3 / t309 / t130 / f64x8::splat(3.0);
            let t317 = t315 * t59 * t304;
            let t318 = t3 * t307;
            let t319 = t129 * t129;
            let t320 = f64x8::splat(1.0) / t319;
            let t321 = t308 * t320;
            let t322 = t149 * t149;
            let t328 = t125 * t156 * f64x8::splat(M_PI);
            let t329 = t128 * t25;
            let t330 = t130 * t149;
            let t335 = t129 * t49;
            let t336 = f64x8::splat(1.0) / t335;
            let t337 = t25 * t336;
            let t342 = t30 * v_rho;
            let t343 = t31 * t342;
            let t345 = f64x8::splat(1.0) / t19 / t343;
            let t347 = t29 * t345 * t45;
            let t350 = t141 * t30;
            let t351 = f64x8::splat(1.0) / t350;
            let t352 = t351 * t145;
            let t355 = t27 * t27;
            let t356 = param_b * t355;
            let t357 = t141 * t31;
            let t359 = f64x8::splat(1.0) / t38 / t357;
            let t360 = t44 * t44;
            let t361 = f64x8::splat(1.0) / t360;
            let t363 = t359 * t361 * t36;
            let t366 = f64x8::splat(2432.0) * t28 * t347 - f64x8::splat(66048.0) * t140 * t352 + f64x8::splat(221184.0) * t356 * t363;
            let t371 = t11 * t11;
            let t374 = f64x8::splat(1.0) / t57 / t371 / t30;
            let t375 = t29 * t374;
            let t379 = f64x8::splat(9.0) / f64x8::splat(8.0) * t317 * t318 * t321 * t322 - t328 * t329 * t330 * t11 / f64x8::splat(6.0) - t127 * t128 * t337 * t322 / f64x8::splat(2.0) + t127 * t128 * t131 * t366 / f64x8::splat(4.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t55 * t375 * t371;
            let t380 = ((t63).select(t379, f64x8::splat(0.0)));
            let t408 = f64x8::splat(1.0) / t86 / t65;
            let t413 = t69 * t301 / f64x8::splat(6.0) - t122 * t380 / f64x8::splat(18.0) - t72 * t301 / f64x8::splat(48.0) + t165 * t380 / f64x8::splat(240.0) + t75 * t301 / f64x8::splat(640.0) - t169 * t380 / f64x8::splat(4480.0) - t78 * t301 / f64x8::splat(11520.0) + t173 * t380 / f64x8::splat(103680.0) + t81 * t301 / f64x8::splat(258048.0) - t177 * t380 / f64x8::splat(2838528.0) - t84 * t301 / f64x8::splat(6881280.0) + t181 * t380 / f64x8::splat(89456640.0) + t87 * t301 / f64x8::splat(212336640.0) - t185 * t380 / f64x8::splat(3185049600.0) - t408 * t301 / f64x8::splat(7431782400.0) + t189 * t380 / f64x8::splat(126340300800.0);
            let t414 = ((t63).select(f64x8::splat(0.0), t379));
            let t419 = t96 * t96;
            let t421 = f64x8::splat(1.0) / t419 / t90;
            let t422 = t193 * t193;
            let t423 = t421 * t422;
            let t426 = t99 * t200;
            let t434 = f64x8::splat(1.0) / t419;
            let t442 = f64x8::splat(1.0) / t419 / t96;
            let t443 = t442 * t422;
            let t454 = -f64x8::splat(2.0) * t434 * t422 * t99 + t200 * t414 * t99 / f64x8::splat(2.0) + t443 * t99 / f64x8::splat(4.0) - f64x8::splat(4.0) * t422 * t100 - t97 * t422 * t99 - f64x8::splat(4.0) * t204 * t414 - t92 * t414 * t99;
            let t457 = -t423 * t99 / f64x8::splat(2.0) + f64x8::splat(2.0) * t426 * t422 - t195 * t414 + f64x8::splat(2.0) * t414 * t103 + f64x8::splat(4.0) * t193 * t209 + f64x8::splat(2.0) * t90 * t454;
            let t461 = ((t62).select(t413, -f64x8::splat(8.0) / f64x8::splat(3.0) * t414 * t106 - f64x8::splat(16.0) / f64x8::splat(3.0) * t193 * t212 - f64x8::splat(8.0) / f64x8::splat(3.0) * t90 * t457));
            let t462 = t19 * t461;
            let t473 = ((t2).select(f64x8::splat(0.0), t18 * t290 * t49 / f64x8::splat(12.0) - t18 * t294 * t49 / f64x8::splat(4.0) - t18 * t117 * t149 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t462 * t49 - f64x8::splat(3.0) / f64x8::splat(4.0) * t18 * t217 * t149 - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t111 * t366));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t473 + f64x8::splat(4.0) * t225;
            acc_v2rho2 = tv2rho20;
            let t476 = t116 * t277;
            let t480 = t69 * t240;
            let t483 = t318 * t308;
            let t484 = t320 * t235;
            let t489 = t130 * t235;
            let t494 = t336 * t235;
            let t503 = t141 * t342;
            let t505 = f64x8::splat(1.0) / t38 / t503;
            let t507 = t505 * t361 * t36;
            let t510 = -f64x8::splat(768.0) * t228 * t136 + f64x8::splat(23040.0) * t28 * t146 - f64x8::splat(82944.0) * t140 * t507;
            let t515 = f64x8::splat(9.0) / f64x8::splat(8.0) * t317 * t483 * t484 * t149 - t328 * t329 * t489 * t11 / f64x8::splat(12.0) - t127 * t329 * t494 * t149 / f64x8::splat(2.0) + t127 * t128 * t131 * t510 / f64x8::splat(4.0);
            let t516 = ((t63).select(t515, f64x8::splat(0.0)));
            let t519 = t72 * t240;
            let t522 = t165 * t516;
            let t524 = t75 * t240;
            let t527 = t169 * t516;
            let t529 = t78 * t240;
            let t532 = t173 * t516;
            let t534 = t81 * t240;
            let t537 = t177 * t516;
            let t539 = t84 * t240;
            let t542 = t181 * t516;
            let t544 = t87 * t240;
            let t547 = t185 * t516;
            let t549 = t408 * t240;
            let t552 = t189 * t516;
            let t554 = t480 * t161 / f64x8::splat(6.0) - t122 * t516 / f64x8::splat(18.0) - t519 * t161 / f64x8::splat(48.0) + t522 / f64x8::splat(240.0) + t524 * t161 / f64x8::splat(640.0) - t527 / f64x8::splat(4480.0) - t529 * t161 / f64x8::splat(11520.0) + t532 / f64x8::splat(103680.0) + t534 * t161 / f64x8::splat(258048.0) - t537 / f64x8::splat(2838528.0) - t539 * t161 / f64x8::splat(6881280.0) + t542 / f64x8::splat(89456640.0) + t544 * t161 / f64x8::splat(212336640.0) - t547 / f64x8::splat(3185049600.0) - t549 * t161 / f64x8::splat(7431782400.0) + t552 / f64x8::splat(126340300800.0);
            let t555 = ((t63).select(f64x8::splat(0.0), t515));
            let t559 = t421 * t193;
            let t560 = t99 * t258;
            let t563 = t258 * t193;
            let t573 = t434 * t258;
            let t574 = t99 * t193;
            let t580 = t442 * t258;
            let t583 = t193 * t100;
            let t586 = t97 * t193;
            let t592 = -f64x8::splat(2.0) * t573 * t574 + t200 * t555 * t99 / f64x8::splat(2.0) + t580 * t574 / f64x8::splat(4.0) - f64x8::splat(4.0) * t583 * t258 - t586 * t560 - f64x8::splat(4.0) * t204 * t555 - t92 * t555 * t99;
            let t595 = -t559 * t560 / f64x8::splat(2.0) + f64x8::splat(2.0) * t426 * t563 - t195 * t555 + f64x8::splat(2.0) * t555 * t103 + f64x8::splat(2.0) * t258 * t209 + f64x8::splat(2.0) * t193 * t270 + f64x8::splat(2.0) * t90 * t592;
            let t599 = ((t62).select(t554, -f64x8::splat(8.0) / f64x8::splat(3.0) * t555 * t106 - f64x8::splat(8.0) / f64x8::splat(3.0) * t193 * t273 - f64x8::splat(8.0) / f64x8::splat(3.0) * t258 * t212 - f64x8::splat(8.0) / f64x8::splat(3.0) * t90 * t595));
            let t600 = t19 * t599;
            let t617 = ((t2).select(f64x8::splat(0.0), -t18 * t476 * t49 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t600 * t49 - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t278 * t149 - t18 * t117 * t235 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t217 * t235 - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t111 * t510));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t617 + f64x8::splat(2.0) * t285;
            acc_v2rhosigma = tv2rhosigma0;
            let t620 = t240 * t240;
            let t623 = t235 * t235;
            let t632 = param_b * t29;
            let t639 = f64x8::splat(1.0) / t38 / t350;
            let t641 = t639 * t361 * t36;
            let t644 = f64x8::splat(144.0) * t632 * t34 * t45 - f64x8::splat(6912.0) * t228 * t232 + f64x8::splat(31104.0) * t28 * t641;
            let t649 = f64x8::splat(9.0) / f64x8::splat(8.0) * t317 * t318 * t321 * t623 - t127 * t128 * t337 * t623 / f64x8::splat(2.0) + t127 * t128 * t131 * t644 / f64x8::splat(4.0);
            let t650 = ((t63).select(t649, f64x8::splat(0.0)));
            let t653 = t72 * t620;
            let t655 = t165 * t650;
            let t657 = t75 * t620;
            let t659 = t169 * t650;
            let t661 = t78 * t620;
            let t663 = t173 * t650;
            let t665 = t81 * t620;
            let t667 = t177 * t650;
            let t669 = t84 * t620;
            let t671 = t181 * t650;
            let t673 = t87 * t620;
            let t675 = t185 * t650;
            let t677 = t408 * t620;
            let t679 = t189 * t650;
            let t681 = t69 * t620 / f64x8::splat(6.0) - t122 * t650 / f64x8::splat(18.0) - t653 / f64x8::splat(48.0) + t655 / f64x8::splat(240.0) + t657 / f64x8::splat(640.0) - t659 / f64x8::splat(4480.0) - t661 / f64x8::splat(11520.0) + t663 / f64x8::splat(103680.0) + t665 / f64x8::splat(258048.0) - t667 / f64x8::splat(2838528.0) - t669 / f64x8::splat(6881280.0) + t671 / f64x8::splat(89456640.0) + t673 / f64x8::splat(212336640.0) - t675 / f64x8::splat(3185049600.0) - t677 / f64x8::splat(7431782400.0) + t679 / f64x8::splat(126340300800.0);
            let t682 = ((t63).select(f64x8::splat(0.0), t649));
            let t687 = t258 * t258;
            let t688 = t421 * t687;
            let t704 = t442 * t687;
            let t715 = -f64x8::splat(2.0) * t434 * t687 * t99 + t200 * t682 * t99 / f64x8::splat(2.0) + t704 * t99 / f64x8::splat(4.0) - f64x8::splat(4.0) * t687 * t100 - t97 * t687 * t99 - f64x8::splat(4.0) * t204 * t682 - t92 * t682 * t99;
            let t718 = -t688 * t99 / f64x8::splat(2.0) + f64x8::splat(2.0) * t426 * t687 - t195 * t682 + f64x8::splat(2.0) * t682 * t103 + f64x8::splat(4.0) * t258 * t270 + f64x8::splat(2.0) * t90 * t715;
            let t722 = ((t62).select(t681, -f64x8::splat(8.0) / f64x8::splat(3.0) * t682 * t106 - f64x8::splat(16.0) / f64x8::splat(3.0) * t258 * t273 - f64x8::splat(8.0) / f64x8::splat(3.0) * t90 * t718));
            let t723 = t19 * t722;
            let t734 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t723 * t49 - f64x8::splat(3.0) / f64x8::splat(4.0) * t18 * t278 * t235 - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t111 * t644));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t734;
            acc_v2sigma2 = tv2sigma20;
            let t737 = t40 * t110;
            let t741 = t289 * t216;
            let t748 = t116 * t461;
            let t758 = t304 * t304;
            let t763 = param_hyb_omega_0 / t53 / t758 / t336 / f64x8::splat(36.0);
            let t764 = t763 * t29;
            let t765 = t58 * t758;
            let t767 = f64x8::splat(1.0) / t319 / t129;
            let t768 = t322 * t149;
            let t774 = t315 * t156 * t304;
            let t775 = t320 * t322;
            let t781 = f64x8::splat(1.0) / t319 / t49;
            let t782 = t308 * t781;
            let t787 = t320 * t149;
            let t793 = t125 * t375 * f64x8::splat(M_PI);
            let t798 = t336 * t322;
            let t803 = t130 * t366;
            let t808 = t25 * t320;
            let t813 = t336 * t149;
            let t821 = t29 / t19 / t141 * t45;
            let t825 = f64x8::splat(1.0) / t503 * t145;
            let t828 = t141 * t32;
            let t832 = f64x8::splat(1.0) / t38 / t828 * t361 * t36;
            let t836 = param_b * t355 * v_sigma;
            let t837 = t141 * t141;
            let t841 = f64x8::splat(1.0) / t360 / t43;
            let t843 = f64x8::splat(1.0) / t19 / t837 * t841 * t29;
            let t846 = -f64x8::splat(17834.666666666668) * t28 * t821 + f64x8::splat(816128.0) * t140 * t825 - f64x8::splat(5971968.0) * t356 * t832 + f64x8::splat(28311552.0) * t836 * t843;
            let t851 = t371 * t11;
            let t855 = t29 / t57 / t851 / t342;
            let t859 = f64x8::splat(135.0) / f64x8::splat(4.0) * t764 * t765 * t767 * t768 - f64x8::splat(9.0) / f64x8::splat(8.0) * t774 * t483 * t775 * t11 - f64x8::splat(27.0) / f64x8::splat(4.0) * t317 * t318 * t782 * t768 + f64x8::splat(27.0) / f64x8::splat(8.0) * t317 * t483 * t787 * t366 + t793 * t329 * t330 * t371 / f64x8::splat(3.0) + t328 * t329 * t798 * t11 / f64x8::splat(2.0) - t328 * t329 * t803 * t11 / f64x8::splat(4.0) + f64x8::splat(3.0) / f64x8::splat(2.0) * t127 * t128 * t808 * t768 - f64x8::splat(3.0) / f64x8::splat(2.0) * t127 * t329 * t813 * t366 + t127 * t128 * t131 * t846 / f64x8::splat(4.0) - f64x8::splat(14.0) / f64x8::splat(27.0) * t55 * t855 * t851;
            let t860 = ((t63).select(t859, f64x8::splat(0.0)));
            let t878 = f64x8::splat(1.0) / t86 / t121;
            let t879 = t301 * t161;
            let t890 = t189 * t860 / f64x8::splat(126340300800.0) - t185 * t860 / f64x8::splat(3185049600.0) + t181 * t860 / f64x8::splat(89456640.0) - t177 * t860 / f64x8::splat(2838528.0) + t173 * t860 / f64x8::splat(103680.0) - t169 * t860 / f64x8::splat(4480.0) + t165 * t860 / f64x8::splat(240.0) - t122 * t860 / f64x8::splat(18.0) + t878 * t879 / f64x8::splat(412876800.0) - t408 * t161 * t380 / f64x8::splat(2477260800.0) - t84 * t161 * t380 / f64x8::splat(2293760.0) - t189 * t879 / f64x8::splat(13271040.0);
            let t921 = t87 * t161 * t380 / f64x8::splat(70778880.0) - t181 * t879 / f64x8::splat(21504.0) + t81 * t161 * t380 / f64x8::splat(86016.0) + t185 * t879 / f64x8::splat(491520.0) + t177 * t879 / f64x8::splat(1152.0) - t78 * t161 * t380 / f64x8::splat(3840.0) + t169 * t879 / f64x8::splat(8.0) - t72 * t161 * t380 / f64x8::splat(16.0) - t173 * t879 / f64x8::splat(80.0) + f64x8::splat(3.0) / f64x8::splat(640.0) * t75 * t161 * t380 - f64x8::splat(2.0) / f64x8::splat(3.0) * t165 * t879 + t69 * t161 * t380 / f64x8::splat(2.0);
            let t923 = ((t63).select(f64x8::splat(0.0), t859));
            let t930 = t422 * t193;
            let t934 = t99 * t414;
            let t937 = t419 * t419;
            let t938 = f64x8::splat(1.0) / t937;
            let t942 = t99 * t434;
            let t945 = t193 * t414;
            let t958 = t434 * t193;
            let t962 = f64x8::splat(1.0) / t419 / t199;
            let t963 = t962 * t930;
            let t973 = f64x8::splat(1.0) / t937 / t90;
            let t974 = t973 * t930;
            let t985 = f64x8::splat(15.0) / f64x8::splat(2.0) * t421 * t930 * t99 - f64x8::splat(6.0) * t958 * t934 - f64x8::splat(5.0) / f64x8::splat(2.0) * t963 * t99 + t200 * t923 * t99 / f64x8::splat(2.0) + f64x8::splat(3.0) / f64x8::splat(4.0) * t442 * t414 * t574 + t974 * t99 / f64x8::splat(8.0) - f64x8::splat(12.0) * t583 * t414 - f64x8::splat(3.0) * t586 * t934 - f64x8::splat(4.0) * t204 * t923 - t92 * t923 * t99;
            let t988 = f64x8::splat(7.0) / f64x8::splat(2.0) * t442 * t930 * t99 - f64x8::splat(3.0) / f64x8::splat(2.0) * t559 * t934 - t938 * t930 * t99 / f64x8::splat(4.0) - f64x8::splat(6.0) * t942 * t930 + f64x8::splat(6.0) * t426 * t945 - t195 * t923 + f64x8::splat(2.0) * t923 * t103 + f64x8::splat(6.0) * t414 * t209 + f64x8::splat(6.0) * t193 * t454 + f64x8::splat(2.0) * t90 * t985;
            let t992 = ((t62).select(t890 + t921, -f64x8::splat(8.0) / f64x8::splat(3.0) * t923 * t106 - f64x8::splat(8.0) * t414 * t212 - f64x8::splat(8.0) * t193 * t457 - f64x8::splat(8.0) / f64x8::splat(3.0) * t90 * t988));
            let t993 = t19 * t992;
            let t1007 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(5.0) / f64x8::splat(36.0) * t18 * t737 * t49 + t18 * t741 * t49 / f64x8::splat(4.0) + t18 * t290 * t149 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t748 * t49 - f64x8::splat(3.0) / f64x8::splat(4.0) * t18 * t294 * t149 - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t117 * t366 - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t993 * t49 - f64x8::splat(9.0) / f64x8::splat(8.0) * t18 * t462 * t149 - f64x8::splat(9.0) / f64x8::splat(8.0) * t18 * t217 * t366 - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t111 * t846));
            let tv3rho30 = f64x8::splat(2.0) * v_rho * t1007 + f64x8::splat(6.0) * t473;
            acc_v3rho3 = tv3rho30;
            let t1011 = t289 * t277;
            let t1015 = t116 * t599;
            let t1028 = t878 * t240;
            let t1041 = t81 * t516;
            let t1046 = t78 * t516;
            let t1051 = t72 * t516;
            let t1056 = t75 * t516;
            let t1059 = t480 * t380 / f64x8::splat(6.0) + t245 * t301 / f64x8::splat(8.0) - f64x8::splat(2.0) / f64x8::splat(3.0) * t243 * t301 + t1028 * t301 / f64x8::splat(412876800.0) - t255 * t301 / f64x8::splat(13271040.0) + t253 * t301 / f64x8::splat(491520.0) - t251 * t301 / f64x8::splat(21504.0) + t249 * t301 / f64x8::splat(1152.0) - t247 * t301 / f64x8::splat(80.0) + t1041 * t161 / f64x8::splat(129024.0) + t534 * t380 / f64x8::splat(258048.0) - t1046 * t161 / f64x8::splat(5760.0) - t529 * t380 / f64x8::splat(11520.0) - t1051 * t161 / f64x8::splat(24.0) - t519 * t380 / f64x8::splat(48.0) + t1056 * t161 / f64x8::splat(320.0);
            let t1062 = t69 * t516;
            let t1065 = t87 * t516;
            let t1070 = t408 * t516;
            let t1075 = t84 * t516;
            let t1080 = t763 * t59;
            let t1081 = t758 * t767;
            let t1082 = t235 * t322;
            let t1086 = t315 * t29;
            let t1089 = t1086 * t155 * t304 * t3;
            let t1090 = t309 * t320;
            let t1091 = t235 * t149;
            let t1092 = t1091 * t11;
            let t1096 = t781 * t235;
            let t1101 = t320 * t510;
            let t1114 = t125 * t29;
            let t1117 = t1114 * t155 * f64x8::splat(M_PI) * t20;
            let t1118 = t26 * t336;
            let t1122 = t130 * t510;
            let t1131 = t336 * t510;
            let t1149 = f64x8::splat(1.0) / t19 / t141 / t343 * t841 * t29;
            let t1152 = f64x8::splat(4864.0) * t228 * t347 - f64x8::splat(256512.0) * t28 * t352 + f64x8::splat(2073600.0) * t140 * t363 - f64x8::splat(10616832.0) * t356 * t1149;
            let t1157 = f64x8::splat(135.0) / f64x8::splat(4.0) * t1080 * t1081 * t1082 - f64x8::splat(3.0) / f64x8::splat(4.0) * t1089 * t1090 * t1092 - f64x8::splat(27.0) / f64x8::splat(4.0) * t317 * t483 * t1096 * t322 + f64x8::splat(9.0) / f64x8::splat(4.0) * t317 * t483 * t1101 * t149 + f64x8::splat(9.0) / f64x8::splat(8.0) * t317 * t483 * t484 * t366 + t793 * t329 * t489 * t371 / f64x8::splat(9.0) + t1117 * t1118 * t1092 / f64x8::splat(3.0) - t328 * t329 * t1122 * t11 / f64x8::splat(6.0) + f64x8::splat(3.0) / f64x8::splat(2.0) * t127 * t329 * t484 * t322 - t127 * t329 * t1131 * t149 - t127 * t329 * t494 * t366 / f64x8::splat(2.0) + t127 * t128 * t131 * t1152 / f64x8::splat(4.0);
            let t1158 = ((t63).select(t1157, f64x8::splat(0.0)));
            let t1175 = t524 * t380 / f64x8::splat(640.0) + t1062 * t161 / f64x8::splat(3.0) + t1065 * t161 / f64x8::splat(106168320.0) + t544 * t380 / f64x8::splat(212336640.0) - t1070 * t161 / f64x8::splat(3715891200.0) - t549 * t380 / f64x8::splat(7431782400.0) - t1075 * t161 / f64x8::splat(3440640.0) - t539 * t380 / f64x8::splat(6881280.0) + t189 * t1158 / f64x8::splat(126340300800.0) - t185 * t1158 / f64x8::splat(3185049600.0) + t181 * t1158 / f64x8::splat(89456640.0) - t177 * t1158 / f64x8::splat(2838528.0) + t173 * t1158 / f64x8::splat(103680.0) - t169 * t1158 / f64x8::splat(4480.0) + t165 * t1158 / f64x8::splat(240.0) - t122 * t1158 / f64x8::splat(18.0);
            let t1177 = ((t63).select(f64x8::splat(0.0), t1157));
            let t1190 = t421 * t414;
            let t1193 = t938 * t422;
            let t1196 = t99 * t555;
            let t1218 = t421 * t258;
            let t1219 = t99 * t422;
            let t1222 = t434 * t555;
            let t1225 = t962 * t258;
            let t1233 = t442 * t555;
            let t1238 = t973 * t258;
            let t1241 = t414 * t100;
            let t1246 = t97 * t414;
            let t1254 = f64x8::splat(15.0) / f64x8::splat(2.0) * t1218 * t1219 - f64x8::splat(4.0) * t1222 * t574 - f64x8::splat(5.0) / f64x8::splat(2.0) * t1225 * t1219 - f64x8::splat(2.0) * t573 * t934 + t200 * t1177 * t99 / f64x8::splat(2.0) + t1233 * t574 / f64x8::splat(2.0) + t580 * t934 / f64x8::splat(4.0) + t1238 * t1219 / f64x8::splat(8.0) - f64x8::splat(4.0) * t1241 * t258 - f64x8::splat(8.0) * t583 * t555 - t1246 * t560 - f64x8::splat(2.0) * t586 * t1196 - f64x8::splat(4.0) * t204 * t1177 - t92 * t1177 * t99;
            let t1257 = f64x8::splat(7.0) / f64x8::splat(2.0) * t443 * t560 - t1190 * t560 / f64x8::splat(2.0) - t1193 * t560 / f64x8::splat(4.0) - t559 * t1196 - f64x8::splat(6.0) * t942 * t258 * t422 + f64x8::splat(4.0) * t426 * t555 * t193 + f64x8::splat(2.0) * t426 * t258 * t414 - t195 * t1177 + f64x8::splat(2.0) * t1177 * t103 + f64x8::splat(4.0) * t555 * t209 + f64x8::splat(2.0) * t258 * t454 + f64x8::splat(2.0) * t414 * t270 + f64x8::splat(4.0) * t193 * t592 + f64x8::splat(2.0) * t90 * t1254;
            let t1261 = ((t62).select(t1059 + t1175, -f64x8::splat(8.0) / f64x8::splat(3.0) * t1177 * t106 - f64x8::splat(16.0) / f64x8::splat(3.0) * t555 * t212 - f64x8::splat(8.0) / f64x8::splat(3.0) * t258 * t457 - f64x8::splat(8.0) / f64x8::splat(3.0) * t414 * t273 - f64x8::splat(16.0) / f64x8::splat(3.0) * t193 * t595 - f64x8::splat(8.0) / f64x8::splat(3.0) * t90 * t1257));
            let t1262 = t19 * t1261;
            let t1290 = t18 * t1011 * t49 / f64x8::splat(12.0) - t18 * t1015 * t49 / f64x8::splat(4.0) - t18 * t476 * t149 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t1262 * t49 - f64x8::splat(3.0) / f64x8::splat(4.0) * t18 * t600 * t149 - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t278 * t366 + t18 * t290 * t235 / f64x8::splat(12.0) - t18 * t294 * t235 / f64x8::splat(4.0) - t18 * t117 * t510 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t462 * t235 - f64x8::splat(3.0) / f64x8::splat(4.0) * t18 * t217 * t510 - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t111 * t1152;
            let t1291 = ((t2).select(f64x8::splat(0.0), t1290));
            let tv3rho2sigma0 = f64x8::splat(2.0) * v_rho * t1291 + f64x8::splat(4.0) * t617;
            acc_v3rho2sigma = tv3rho2sigma0;
            let t1294 = t116 * t722;
            let t1298 = t185 * t620;
            let t1303 = t84 * t650;
            let t1306 = t189 * t620;
            let t1311 = t87 * t650;
            let t1314 = t878 * t620;
            let t1319 = t408 * t650;
            let t1322 = t177 * t620;
            let t1327 = t78 * t650;
            let t1330 = t181 * t620;
            let t1335 = t81 * t650;
            let t1340 = t1298 * t161 / f64x8::splat(491520.0) - t539 * t516 / f64x8::splat(3440640.0) - t1303 * t161 / f64x8::splat(6881280.0) - t1306 * t161 / f64x8::splat(13271040.0) + t544 * t516 / f64x8::splat(106168320.0) + t1311 * t161 / f64x8::splat(212336640.0) + t1314 * t161 / f64x8::splat(412876800.0) - t549 * t516 / f64x8::splat(3715891200.0) - t1319 * t161 / f64x8::splat(7431782400.0) + t1322 * t161 / f64x8::splat(1152.0) - t529 * t516 / f64x8::splat(5760.0) - t1327 * t161 / f64x8::splat(11520.0) - t1330 * t161 / f64x8::splat(21504.0) + t534 * t516 / f64x8::splat(129024.0) + t1335 * t161 / f64x8::splat(258048.0) + t480 * t516 / f64x8::splat(3.0);
            let t1341 = t69 * t650;
            let t1344 = t169 * t620;
            let t1349 = t72 * t650;
            let t1352 = t173 * t620;
            let t1357 = t75 * t650;
            let t1360 = t165 * t620;
            let t1363 = t623 * t149;
            let t1367 = t320 * t623;
            let t1372 = t781 * t623;
            let t1381 = t336 * t623;
            let t1393 = t320 * t644;
            let t1398 = t130 * t644;
            let t1403 = t336 * t644;
            let t1411 = param_b * t143;
            let t1412 = t145 * v_sigma;
            let t1417 = t141 * t132;
            let t1421 = f64x8::splat(1.0) / t19 / t1417 * t841 * t29;
            let t1424 = -f64x8::splat(768.0) * t632 * t134 * t45 + f64x8::splat(64512.0) * t1411 * t1412 - f64x8::splat(663552.0) * t28 * t507 + f64x8::splat(3981312.0) * t140 * t1421;
            let t1429 = f64x8::splat(135.0) / f64x8::splat(4.0) * t1080 * t1081 * t1363 - f64x8::splat(3.0) / f64x8::splat(8.0) * t774 * t483 * t1367 * t11 - f64x8::splat(27.0) / f64x8::splat(4.0) * t317 * t483 * t1372 * t149 + f64x8::splat(9.0) / f64x8::splat(4.0) * t317 * t483 * t484 * t510 + t328 * t329 * t1381 * t11 / f64x8::splat(6.0) + f64x8::splat(3.0) / f64x8::splat(2.0) * t127 * t329 * t1367 * t149 - t127 * t329 * t494 * t510 + f64x8::splat(9.0) / f64x8::splat(8.0) * t317 * t483 * t1393 * t149 - t328 * t329 * t1398 * t11 / f64x8::splat(12.0) - t127 * t329 * t1403 * t149 / f64x8::splat(2.0) + t127 * t128 * t131 * t1424 / f64x8::splat(4.0);
            let t1430 = ((t63).select(t1429, f64x8::splat(0.0)));
            let t1447 = t1341 * t161 / f64x8::splat(6.0) + t1344 * t161 / f64x8::splat(8.0) - t519 * t516 / f64x8::splat(24.0) - t1349 * t161 / f64x8::splat(48.0) - t1352 * t161 / f64x8::splat(80.0) + t524 * t516 / f64x8::splat(320.0) + t1357 * t161 / f64x8::splat(640.0) - f64x8::splat(2.0) / f64x8::splat(3.0) * t1360 * t161 + t189 * t1430 / f64x8::splat(126340300800.0) - t185 * t1430 / f64x8::splat(3185049600.0) + t181 * t1430 / f64x8::splat(89456640.0) - t177 * t1430 / f64x8::splat(2838528.0) + t173 * t1430 / f64x8::splat(103680.0) - t169 * t1430 / f64x8::splat(4480.0) + t165 * t1430 / f64x8::splat(240.0) - t122 * t1430 / f64x8::splat(18.0);
            let t1449 = ((t63).select(f64x8::splat(0.0), t1429));
            let t1463 = t938 * t687;
            let t1472 = t99 * t682;
            let t1493 = t962 * t687;
            let t1496 = t434 * t682;
            let t1502 = t442 * t682;
            let t1507 = t973 * t687;
            let t1510 = t258 * t100;
            let t1513 = t97 * t258;
            let t1523 = f64x8::splat(15.0) / f64x8::splat(2.0) * t688 * t574 - f64x8::splat(4.0) * t573 * t1196 - f64x8::splat(5.0) / f64x8::splat(2.0) * t1493 * t574 - f64x8::splat(2.0) * t1496 * t574 + t200 * t1449 * t99 / f64x8::splat(2.0) + t1502 * t574 / f64x8::splat(4.0) + t580 * t1196 / f64x8::splat(2.0) + t1507 * t574 / f64x8::splat(8.0) - f64x8::splat(8.0) * t1510 * t555 - f64x8::splat(2.0) * t1513 * t1196 - f64x8::splat(4.0) * t583 * t682 - t586 * t1472 - f64x8::splat(4.0) * t204 * t1449 - t92 * t1449 * t99;
            let t1526 = f64x8::splat(7.0) / f64x8::splat(2.0) * t704 * t574 - t1218 * t1196 - t1463 * t574 / f64x8::splat(4.0) - f64x8::splat(6.0) * t942 * t687 * t193 + f64x8::splat(4.0) * t426 * t258 * t555 - t559 * t1472 / f64x8::splat(2.0) + f64x8::splat(2.0) * t426 * t682 * t193 - t195 * t1449 + f64x8::splat(2.0) * t1449 * t103 + f64x8::splat(2.0) * t682 * t209 + f64x8::splat(4.0) * t555 * t270 + f64x8::splat(4.0) * t258 * t592 + f64x8::splat(2.0) * t193 * t715 + f64x8::splat(2.0) * t90 * t1523;
            let t1530 = ((t62).select(t1340 + t1447, -f64x8::splat(8.0) / f64x8::splat(3.0) * t1449 * t106 - f64x8::splat(8.0) / f64x8::splat(3.0) * t682 * t212 - f64x8::splat(16.0) / f64x8::splat(3.0) * t555 * t273 - f64x8::splat(16.0) / f64x8::splat(3.0) * t258 * t595 - f64x8::splat(8.0) / f64x8::splat(3.0) * t193 * t718 - f64x8::splat(8.0) / f64x8::splat(3.0) * t90 * t1526));
            let t1531 = t19 * t1530;
            let t1557 = ((t2).select(f64x8::splat(0.0), -t18 * t1294 * t49 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t1531 * t49 - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t723 * t149 - t18 * t476 * t235 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t18 * t600 * t235 - f64x8::splat(3.0) / f64x8::splat(4.0) * t18 * t278 * t510 - t18 * t117 * t644 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t217 * t644 - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t111 * t1424));
            let tv3rhosigma20 = f64x8::splat(2.0) * v_rho * t1557 + f64x8::splat(2.0) * t734;
            acc_v3rhosigma2 = tv3rhosigma20;
            let t1560 = t620 * t240;
            let t1585 = t185 * t1560 / f64x8::splat(491520.0) - t539 * t650 / f64x8::splat(2293760.0) - t189 * t1560 / f64x8::splat(13271040.0) + t544 * t650 / f64x8::splat(70778880.0) + t878 * t1560 / f64x8::splat(412876800.0) - t549 * t650 / f64x8::splat(2477260800.0) + f64x8::splat(3.0) / f64x8::splat(640.0) * t524 * t650 + t177 * t1560 / f64x8::splat(1152.0) - t529 * t650 / f64x8::splat(3840.0) - t181 * t1560 / f64x8::splat(21504.0) + t534 * t650 / f64x8::splat(86016.0) - f64x8::splat(2.0) / f64x8::splat(3.0) * t165 * t1560;
            let t1594 = t623 * t235;
            let t1595 = t767 * t1594;
            let t1623 = f64x8::splat(1.0) / t19 / t828 * t841 * t29;
            let t1626 = -f64x8::splat(10368.0) * param_b * t231 * t145 + f64x8::splat(186624.0) * t228 * t641 - f64x8::splat(1492992.0) * t28 * t1623;
            let t1631 = f64x8::splat(135.0) / f64x8::splat(4.0) * t764 * t765 * t1595 - f64x8::splat(27.0) / f64x8::splat(4.0) * t317 * t318 * t782 * t1594 + f64x8::splat(27.0) / f64x8::splat(8.0) * t317 * t483 * t484 * t644 + f64x8::splat(3.0) / f64x8::splat(2.0) * t127 * t128 * t808 * t1594 - f64x8::splat(3.0) / f64x8::splat(2.0) * t127 * t329 * t494 * t644 + t127 * t128 * t131 * t1626 / f64x8::splat(4.0);
            let t1632 = ((t63).select(t1631, f64x8::splat(0.0)));
            let t1649 = t480 * t650 / f64x8::splat(2.0) + t169 * t1560 / f64x8::splat(8.0) - t519 * t650 / f64x8::splat(16.0) - t173 * t1560 / f64x8::splat(80.0) - t122 * t1632 / f64x8::splat(18.0) + t165 * t1632 / f64x8::splat(240.0) - t169 * t1632 / f64x8::splat(4480.0) + t173 * t1632 / f64x8::splat(103680.0) - t177 * t1632 / f64x8::splat(2838528.0) + t181 * t1632 / f64x8::splat(89456640.0) - t185 * t1632 / f64x8::splat(3185049600.0) + t189 * t1632 / f64x8::splat(126340300800.0);
            let t1651 = ((t63).select(f64x8::splat(0.0), t1631));
            let t1658 = t687 * t258;
            let t1659 = t442 * t1658;
            let t1664 = t938 * t1658;
            let t1669 = t258 * t682;
            let t1684 = t962 * t1658;
            let t1692 = t973 * t1658;
            let t1703 = f64x8::splat(15.0) / f64x8::splat(2.0) * t421 * t1658 * t99 - f64x8::splat(6.0) * t573 * t1472 - f64x8::splat(5.0) / f64x8::splat(2.0) * t1684 * t99 + t200 * t1651 * t99 / f64x8::splat(2.0) + f64x8::splat(3.0) / f64x8::splat(4.0) * t1502 * t560 + t1692 * t99 / f64x8::splat(8.0) - f64x8::splat(12.0) * t1510 * t682 - f64x8::splat(3.0) * t1513 * t1472 - f64x8::splat(4.0) * t204 * t1651 - t92 * t1651 * t99;
            let t1706 = f64x8::splat(7.0) / f64x8::splat(2.0) * t1659 * t99 - f64x8::splat(3.0) / f64x8::splat(2.0) * t1218 * t1472 - t1664 * t99 / f64x8::splat(4.0) - f64x8::splat(6.0) * t942 * t1658 + f64x8::splat(6.0) * t426 * t1669 - t195 * t1651 + f64x8::splat(2.0) * t1651 * t103 + f64x8::splat(6.0) * t682 * t270 + f64x8::splat(6.0) * t258 * t715 + f64x8::splat(2.0) * t90 * t1703;
            let t1710 = ((t62).select(t1585 + t1649, -f64x8::splat(8.0) / f64x8::splat(3.0) * t1651 * t106 - f64x8::splat(8.0) * t682 * t273 - f64x8::splat(8.0) * t258 * t718 - f64x8::splat(8.0) / f64x8::splat(3.0) * t90 * t1706));
            let t1711 = t19 * t1710;
            let t1725 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t1711 * t49 - f64x8::splat(9.0) / f64x8::splat(8.0) * t18 * t723 * t235 - f64x8::splat(9.0) / f64x8::splat(8.0) * t18 * t278 * t644 - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t111 * t1626));
            let tv3sigma30 = f64x8::splat(2.0) * v_rho * t1725;
            acc_v3sigma3 = tv3sigma30;
            let t1785 = t837 * v_rho;
            let t1797 = f64x8::splat(1.0) / t360 / t44;
            let t1801 = f64x8::splat(148622.22222222222) * t28 * t29 / t19 / t142 * t45 - f64x8::splat(10118826.666666666) * t140 / t357 * t145 + f64x8::splat(120791040.0) * t356 / t38 / t1417 * t361 * t36 - f64x8::splat(1226833920.0) * t836 / t19 / t1785 * t841 * t29 + f64x8::splat(4529848320.0) * param_b * t355 * t27 / t837 / t31 * t1797;
            let t1856 = t301 * t301;
            let t1859 = t380 * t380;
            let t1866 = -f64x8::splat(4.0) * t165 * t301 * t380 - f64x8::splat(3.0) / f64x8::splat(40.0) * t173 * t301 * t380 + f64x8::splat(3.0) / f64x8::splat(4.0) * t169 * t301 * t380 + t177 * t301 * t380 / f64x8::splat(192.0) - t181 * t301 * t380 / f64x8::splat(3584.0) - t189 * t301 * t380 / f64x8::splat(2211840.0) + t878 * t301 * t380 / f64x8::splat(68812800.0) + f64x8::splat(2.0) / f64x8::splat(3.0) * t69 * t860 * t161 - t72 * t860 * t161 / f64x8::splat(12.0) + t75 * t860 * t161 / f64x8::splat(160.0) - t78 * t860 * t161 / f64x8::splat(2880.0) + t81 * t860 * t161 / f64x8::splat(64512.0) - t84 * t860 * t161 / f64x8::splat(1720320.0) + t87 * t860 * t161 / f64x8::splat(53084160.0) - t408 * t860 * t161 / f64x8::splat(1857945600.0) + t185 * t301 * t380 / f64x8::splat(81920.0) + f64x8::splat(10.0) / f64x8::splat(3.0) * t72 * t1856 + t69 * t1859 / f64x8::splat(2.0) + f64x8::splat(9.0) / f64x8::splat(80.0) * t78 * t1856 + f64x8::splat(3.0) / f64x8::splat(640.0) * t75 * t1859;
            let t1890 = f64x8::splat(1.0) / t86 / t68;
            let t1893 = t366 * t366;
            let t1907 = t25 * t781;
            let t1908 = t322 * t322;
            let t1917 = t308 * t767;
            let t1951 = param_hyb_omega_0 / t53 / t20 * t23 * t22 / t25 / t320 * t59 * f64x8::splat(M_PI) / f64x8::splat(36.0);
            let t1952 = t319 * t319;
            let t1953 = f64x8::splat(1.0) / t1952;
            let t1958 = -f64x8::splat(3.0) / f64x8::splat(2.0) * t127 * t128 * t337 * t1893 - f64x8::splat(2.0) * t127 * t329 * t813 * t846 - t328 * t329 * t130 * t846 * t11 / f64x8::splat(3.0) - f64x8::splat(6.0) * t127 * t128 * t1907 * t1908 + f64x8::splat(9.0) * t127 * t329 * t775 * t366 + f64x8::splat(81.0) / f64x8::splat(2.0) * t317 * t318 * t1917 * t1908 - f64x8::splat(81.0) / f64x8::splat(2.0) * t317 * t483 * t781 * t322 * t366 + f64x8::splat(27.0) / f64x8::splat(8.0) * t317 * t318 * t321 * t1893 + f64x8::splat(9.0) / f64x8::splat(2.0) * t317 * t483 * t787 * t846 + f64x8::splat(2.0) / f64x8::splat(3.0) * t793 * t329 * t803 * t371 + f64x8::splat(945.0) / f64x8::splat(8.0) * t1951 * t1953 * t1908 * t329;
            let t1969 = t149 * t11 * t366;
            let t1974 = t125 * t855 * f64x8::splat(M_PI);
            let t1984 = t315 * t375 * t304;
            let t1997 = t763 * t156;
            let t2003 = f64x8::splat(1.0) / t319 / t335;
            let t2012 = t371 * t371;
            let t2020 = t127 * t128 * t131 * t1801 / f64x8::splat(4.0) - f64x8::splat(2.0) * t328 * t329 * t320 * t768 * t11 + f64x8::splat(2.0) * t1117 * t1118 * t1969 - f64x8::splat(28.0) / f64x8::splat(27.0) * t1974 * t329 * t330 * t851 - f64x8::splat(4.0) / f64x8::splat(3.0) * t793 * t329 * t798 * t371 + f64x8::splat(3.0) * t1984 * t483 * t775 * t371 + f64x8::splat(9.0) * t774 * t483 * t781 * t768 * t11 - f64x8::splat(9.0) / f64x8::splat(2.0) * t1089 * t1090 * t1969 - f64x8::splat(45.0) * t1997 * t1081 * t768 * t11 - f64x8::splat(405.0) * t764 * t765 * t2003 * t1908 + f64x8::splat(405.0) / f64x8::splat(2.0) * t1080 * t1081 * t322 * t366 + f64x8::splat(140.0) / f64x8::splat(81.0) * t55 * t29 / t57 / t31;
            let t2021 = t1958 + t2020;
            let t2022 = ((t63).select(t2021, f64x8::splat(0.0)));
            let t2039 = -f64x8::splat(7.0) / f64x8::splat(8.0) * t75 * t1856 - t72 * t1859 / f64x8::splat(16.0) - t87 * t1856 / f64x8::splat(32768.0) - f64x8::splat(11.0) / f64x8::splat(1152.0) * t81 * t1856 - t78 * t1859 / f64x8::splat(3840.0) + f64x8::splat(13.0) / f64x8::splat(21504.0) * t84 * t1856 + t81 * t1859 / f64x8::splat(86016.0) + f64x8::splat(17.0) / f64x8::splat(13271040.0) * t408 * t1856 + t87 * t1859 / f64x8::splat(70778880.0) - t408 * t1859 / f64x8::splat(2477260800.0) - t84 * t1859 / f64x8::splat(2293760.0) - f64x8::splat(19.0) / f64x8::splat(412876800.0) * t1890 * t1856 - t122 * t2022 / f64x8::splat(18.0) + t165 * t2022 / f64x8::splat(240.0) - t169 * t2022 / f64x8::splat(4480.0) + t173 * t2022 / f64x8::splat(103680.0) - t177 * t2022 / f64x8::splat(2838528.0) + t181 * t2022 / f64x8::splat(89456640.0) - t185 * t2022 / f64x8::splat(3185049600.0) + t189 * t2022 / f64x8::splat(126340300800.0);
            let t2041 = ((t63).select(f64x8::splat(0.0), t2021));
            let t2050 = t414 * t414;
            let t2054 = t99 * t923;
            let t2066 = f64x8::splat(1.0) / t937 / t419;
            let t2067 = t422 * t422;
            let t2079 = f64x8::splat(1.0) / t937 / t96;
            let t2103 = -f64x8::splat(3.0) * t97 * t2050 * t99 - f64x8::splat(4.0) * t586 * t2054 + f64x8::splat(3.0) / f64x8::splat(4.0) * t442 * t2050 * t99 + f64x8::splat(3.0) / f64x8::splat(4.0) * t973 * t414 * t1219 + t442 * t923 * t574 + t2066 * t2067 * t99 / f64x8::splat(16.0) - f64x8::splat(12.0) * t2050 * t100 - f64x8::splat(16.0) * t583 * t923 + t200 * t2041 * t99 / f64x8::splat(2.0) - f64x8::splat(19.0) / f64x8::splat(8.0) * t2079 * t2067 * t99 + f64x8::splat(85.0) / f64x8::splat(4.0) * t938 * t2067 * t99 - f64x8::splat(15.0) * t962 * t422 * t934 - f64x8::splat(75.0) / f64x8::splat(2.0) * t442 * t2067 * t99 + f64x8::splat(45.0) * t423 * t934 - f64x8::splat(6.0) * t434 * t2050 * t99 - f64x8::splat(8.0) * t958 * t2054 - t92 * t2041 * t99 - f64x8::splat(4.0) * t204 * t2041;
            let t2116 = f64x8::splat(1.0) / t937 / t199;
            let t2123 = t99 * t421;
            let t2146 = f64x8::splat(2.0) * t90 * t2103 + f64x8::splat(8.0) * t193 * t985 + f64x8::splat(12.0) * t414 * t454 + f64x8::splat(8.0) * t923 * t209 + f64x8::splat(2.0) * t2041 * t103 - t195 * t2041 - t2116 * t2067 * t99 / f64x8::splat(8.0) + f64x8::splat(15.0) / f64x8::splat(4.0) * t973 * t2067 * t99 + f64x8::splat(24.0) * t2123 * t2067 - f64x8::splat(36.0) * t942 * t422 * t414 + f64x8::splat(6.0) * t426 * t2050 + f64x8::splat(8.0) * t426 * t193 * t923 - f64x8::splat(2.0) * t559 * t2054 - f64x8::splat(3.0) / f64x8::splat(2.0) * t1193 * t934 - f64x8::splat(24.0) * t962 * t2067 * t99 + f64x8::splat(21.0) * t443 * t934 - f64x8::splat(3.0) / f64x8::splat(2.0) * t421 * t2050 * t99;
            let t2150 = ((t62).select(t1866 + t2039, -f64x8::splat(8.0) / f64x8::splat(3.0) * t2041 * t106 - f64x8::splat(32.0) / f64x8::splat(3.0) * t923 * t212 - f64x8::splat(16.0) * t414 * t457 - f64x8::splat(32.0) / f64x8::splat(3.0) * t193 * t988 - f64x8::splat(8.0) / f64x8::splat(3.0) * t90 * t2146));
            let t2155 = -f64x8::splat(3.0) / f64x8::splat(2.0) * t18 * t294 * t366 + f64x8::splat(10.0) / f64x8::splat(27.0) * t18 / t38 / t342 * t110 * t49 - f64x8::splat(5.0) / f64x8::splat(9.0) * t18 * t737 * t149 + t18 * t289 * t461 * t49 / f64x8::splat(2.0) - f64x8::splat(5.0) / f64x8::splat(9.0) * t18 * t40 * t216 * t49 - f64x8::splat(3.0) / f64x8::splat(2.0) * t18 * t748 * t149 - t18 * t116 * t992 * t49 / f64x8::splat(2.0) + t18 * t290 * t366 / f64x8::splat(2.0) + t18 * t741 * t149 - f64x8::splat(9.0) / f64x8::splat(4.0) * t18 * t462 * t366 - f64x8::splat(3.0) / f64x8::splat(2.0) * t18 * t217 * t846 - f64x8::splat(3.0) / f64x8::splat(2.0) * t18 * t993 * t149 - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t111 * t1801 - t18 * t117 * t846 / f64x8::splat(2.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t19 * t2150 * t49;
            let t2156 = ((t2).select(f64x8::splat(0.0), t2155));
            let tv4rho40 = f64x8::splat(2.0) * v_rho * t2156 + f64x8::splat(8.0) * t1007;
            acc_v4rho4 = tv4rho40;
            let t2173 = -f64x8::splat(35669.333333333336) * t228 * t821 + f64x8::splat(2876416.0) * t28 * t825 - f64x8::splat(38578176.0) * t140 * t832 + f64x8::splat(428212224.0) * t356 * t843 - f64x8::splat(1698693120.0) * t836 / t837 / t342 * t1797;
            let t2229 = t380 * t161;
            let t2242 = -f64x8::splat(7.0) / f64x8::splat(8.0) * t524 * t879 + f64x8::splat(10.0) / f64x8::splat(3.0) * t519 * t879 + t1062 * t380 / f64x8::splat(2.0) + t480 * t860 / f64x8::splat(6.0) + f64x8::splat(3.0) / f64x8::splat(640.0) * t1056 * t380 + t524 * t860 / f64x8::splat(640.0) - t519 * t860 / f64x8::splat(48.0) - t1051 * t380 / f64x8::splat(16.0) - f64x8::splat(2.0) * t243 * t2229 + f64x8::splat(3.0) / f64x8::splat(8.0) * t245 * t2229 + t249 * t2229 / f64x8::splat(384.0) - f64x8::splat(3.0) / f64x8::splat(80.0) * t247 * t2229 - t251 * t2229 / f64x8::splat(7168.0) + t253 * t2229 / f64x8::splat(163840.0);
            let t2247 = t758 * t2003;
            let t2308 = -f64x8::splat(405.0) * t1080 * t2247 * t235 * t768 + f64x8::splat(405.0) / f64x8::splat(4.0) * t1080 * t1081 * t510 * t322 - f64x8::splat(6.0) * t127 * t329 * t1096 * t768 - f64x8::splat(7.0) / f64x8::splat(27.0) * t1974 * t329 * t489 * t851 + f64x8::splat(81.0) / f64x8::splat(2.0) * t317 * t483 * t767 * t235 * t768 - f64x8::splat(3.0) / f64x8::splat(2.0) * t127 * t329 * t336 * t1152 * t149 - f64x8::splat(3.0) / f64x8::splat(2.0) * t127 * t329 * t1131 * t366 - t127 * t329 * t494 * t846 / f64x8::splat(2.0) + f64x8::splat(27.0) / f64x8::splat(8.0) * t317 * t483 * t320 * t1152 * t149 + f64x8::splat(27.0) / f64x8::splat(8.0) * t317 * t483 * t1101 * t366 + f64x8::splat(9.0) / f64x8::splat(8.0) * t317 * t483 * t484 * t846 + t793 * t329 * t1122 * t371 / f64x8::splat(3.0) - t328 * t329 * t130 * t1152 * t11 / f64x8::splat(4.0) + f64x8::splat(9.0) / f64x8::splat(2.0) * t127 * t329 * t1101 * t322;
            let t2319 = t1082 * t11;
            let t2323 = t1091 * t366;
            let t2331 = t235 * t371 * t149;
            let t2336 = t510 * t149 * t11;
            let t2341 = t235 * t366 * t11;
            let t2347 = t1086 * t58 * t304 * t3;
            let t2348 = t309 * t781;
            let t2352 = t26 * t320;
            let t2367 = t1114 * t58 * f64x8::splat(M_PI) * t20;
            let t2380 = f64x8::splat(945.0) / f64x8::splat(8.0) * t1951 * t1953 * t235 * t768 * t329 - f64x8::splat(81.0) / f64x8::splat(4.0) * t317 * t483 * t781 * t510 * t322 - f64x8::splat(135.0) / f64x8::splat(4.0) * t1997 * t1081 * t2319 + f64x8::splat(405.0) / f64x8::splat(4.0) * t1080 * t1081 * t2323 - f64x8::splat(2.0) / f64x8::splat(3.0) * t1114 * t374 * f64x8::splat(M_PI) * t20 * t1118 * t2331 - f64x8::splat(9.0) / f64x8::splat(4.0) * t1089 * t1090 * t2336 - f64x8::splat(9.0) / f64x8::splat(8.0) * t1089 * t1090 * t2341 - f64x8::splat(81.0) / f64x8::splat(4.0) * t2347 * t2348 * t2323 - f64x8::splat(3.0) / f64x8::splat(2.0) * t1117 * t2352 * t2319 + f64x8::splat(3.0) / f64x8::splat(2.0) * t1086 * t374 * t304 * t3 * t1090 * t2331 + f64x8::splat(27.0) / f64x8::splat(4.0) * t1089 * t2348 * t2319 + f64x8::splat(9.0) / f64x8::splat(2.0) * t2367 * t2352 * t2323 + t1117 * t1118 * t2336 + t1117 * t1118 * t2341 / f64x8::splat(2.0) + t127 * t128 * t131 * t2173 / f64x8::splat(4.0);
            let t2381 = t2308 + t2380;
            let t2382 = ((t63).select(t2381, f64x8::splat(0.0)));
            let t2410 = t1028 * t2229 / f64x8::splat(137625600.0) - t255 * t2229 / f64x8::splat(4423680.0) - t122 * t2382 / f64x8::splat(18.0) + t165 * t2382 / f64x8::splat(240.0) - t169 * t2382 / f64x8::splat(4480.0) + t173 * t2382 / f64x8::splat(103680.0) - t177 * t2382 / f64x8::splat(2838528.0) + t181 * t2382 / f64x8::splat(89456640.0) - t185 * t2382 / f64x8::splat(3185049600.0) + t189 * t2382 / f64x8::splat(126340300800.0) - t72 * t1158 * t161 / f64x8::splat(16.0) + f64x8::splat(3.0) / f64x8::splat(640.0) * t75 * t1158 * t161 - t78 * t1158 * t161 / f64x8::splat(3840.0) - f64x8::splat(3.0) / f64x8::splat(80.0) * t532 * t301;
            let t2443 = t81 * t1158 * t161 / f64x8::splat(86016.0) - t542 * t301 / f64x8::splat(7168.0) + t537 * t301 / f64x8::splat(384.0) + f64x8::splat(9.0) / f64x8::splat(80.0) * t529 * t879 - t544 * t879 / f64x8::splat(32768.0) + f64x8::splat(13.0) / f64x8::splat(21504.0) * t539 * t879 - f64x8::splat(11.0) / f64x8::splat(1152.0) * t534 * t879 - f64x8::splat(19.0) / f64x8::splat(412876800.0) * t1890 * t240 * t879 + f64x8::splat(17.0) / f64x8::splat(13271040.0) * t549 * t879 + t547 * t301 / f64x8::splat(163840.0) + t878 * t516 * t301 / f64x8::splat(137625600.0) - t552 * t301 / f64x8::splat(4423680.0) - t1075 * t380 / f64x8::splat(2293760.0) - t539 * t860 / f64x8::splat(6881280.0);
            let t2476 = -t549 * t860 / f64x8::splat(7431782400.0) - t84 * t1158 * t161 / f64x8::splat(2293760.0) - t408 * t1158 * t161 / f64x8::splat(2477260800.0) - t1070 * t380 / f64x8::splat(2477260800.0) + t87 * t1158 * t161 / f64x8::splat(70778880.0) + t1065 * t380 / f64x8::splat(70778880.0) + t544 * t860 / f64x8::splat(212336640.0) - f64x8::splat(2.0) * t522 * t301 + f64x8::splat(3.0) / f64x8::splat(8.0) * t527 * t301 - t1046 * t380 / f64x8::splat(3840.0) - t529 * t860 / f64x8::splat(11520.0) + t1041 * t380 / f64x8::splat(86016.0) + t534 * t860 / f64x8::splat(258048.0) + t69 * t1158 * t161 / f64x8::splat(2.0);
            let t2479 = ((t63).select(f64x8::splat(0.0), t2381));
            let t2498 = t574 * t258;
            let t2525 = t99 * t1177;
            let t2531 = -f64x8::splat(18.0) * t942 * t563 * t414 - f64x8::splat(3.0) / f64x8::splat(4.0) * t938 * t414 * t2498 + f64x8::splat(21.0) / f64x8::splat(2.0) * t442 * t193 * t560 * t414 + f64x8::splat(24.0) * t2123 * t258 * t930 + f64x8::splat(6.0) * t426 * t555 * t414 + f64x8::splat(2.0) * t426 * t258 * t923 - f64x8::splat(18.0) * t942 * t555 * t422 + f64x8::splat(6.0) * t426 * t1177 * t193 - f64x8::splat(3.0) / f64x8::splat(4.0) * t1193 * t1196 - t2116 * t930 * t560 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(2.0) * t559 * t2525 - t421 * t923 * t560 / f64x8::splat(2.0);
            let t2540 = t945 * t99;
            let t2553 = t99 * t930;
            let t2569 = f64x8::splat(3.0) / f64x8::splat(8.0) * t1238 * t2540 - f64x8::splat(15.0) / f64x8::splat(2.0) * t1225 * t2540 + f64x8::splat(45.0) / f64x8::splat(2.0) * t1218 * t2540 - f64x8::splat(3.0) * t586 * t2525 - t97 * t923 * t560 - f64x8::splat(3.0) * t1246 * t1196 - f64x8::splat(75.0) / f64x8::splat(2.0) * t580 * t2553 + t2066 * t258 * t2553 / f64x8::splat(16.0) + f64x8::splat(3.0) / f64x8::splat(4.0) * t1233 * t934 + f64x8::splat(3.0) / f64x8::splat(8.0) * t973 * t555 * t1219 + t580 * t2054 / f64x8::splat(4.0) + f64x8::splat(3.0) / f64x8::splat(4.0) * t442 * t1177 * t574;
            let t2580 = t421 * t555;
            let t2583 = t938 * t258;
            let t2603 = -f64x8::splat(19.0) / f64x8::splat(8.0) * t2079 * t258 * t2553 - f64x8::splat(2.0) * t573 * t2054 - f64x8::splat(6.0) * t1222 * t934 - f64x8::splat(15.0) / f64x8::splat(2.0) * t962 * t555 * t1219 + f64x8::splat(45.0) / f64x8::splat(2.0) * t2580 * t1219 + f64x8::splat(85.0) / f64x8::splat(4.0) * t2583 * t2553 - f64x8::splat(6.0) * t434 * t1177 * t574 - t92 * t2479 * t99 - f64x8::splat(4.0) * t204 * t2479 - f64x8::splat(12.0) * t583 * t1177 - f64x8::splat(4.0) * t923 * t100 * t258 - f64x8::splat(12.0) * t1241 * t555 + t200 * t2479 * t99 / f64x8::splat(2.0);
            let t2622 = -f64x8::splat(3.0) / f64x8::splat(2.0) * t1190 * t1196 + f64x8::splat(21.0) / f64x8::splat(2.0) * t443 * t1196 + f64x8::splat(15.0) / f64x8::splat(4.0) * t974 * t560 - f64x8::splat(24.0) * t963 * t560 + f64x8::splat(2.0) * t90 * (t2569 + t2603) + f64x8::splat(6.0) * t193 * t1254 + f64x8::splat(6.0) * t414 * t592 + f64x8::splat(2.0) * t923 * t270 + f64x8::splat(2.0) * t258 * t985 + f64x8::splat(6.0) * t555 * t454 + f64x8::splat(6.0) * t1177 * t209 + f64x8::splat(2.0) * t2479 * t103 - t195 * t2479;
            let t2627 = ((t62).select(t2242 + t2410 + t2443 + t2476, -f64x8::splat(8.0) / f64x8::splat(3.0) * t2479 * t106 - f64x8::splat(8.0) * t1177 * t212 - f64x8::splat(8.0) * t555 * t457 - f64x8::splat(8.0) / f64x8::splat(3.0) * t258 * t988 - f64x8::splat(8.0) / f64x8::splat(3.0) * t923 * t273 - f64x8::splat(8.0) * t414 * t595 - f64x8::splat(8.0) * t193 * t1257 - f64x8::splat(8.0) / f64x8::splat(3.0) * t90 * (t2531 + t2622)));
            let t2653 = -f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t111 * t2173 - f64x8::splat(9.0) / f64x8::splat(8.0) * t18 * t217 * t1152 - f64x8::splat(9.0) / f64x8::splat(8.0) * t18 * t462 * t510 - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t117 * t1152 - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t993 * t235 - f64x8::splat(3.0) / f64x8::splat(4.0) * t18 * t294 * t510 - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t748 * t235 + t18 * t290 * t510 / f64x8::splat(4.0) + t18 * t741 * t235 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t278 * t846 - f64x8::splat(5.0) / f64x8::splat(36.0) * t18 * t737 * t235 - f64x8::splat(9.0) / f64x8::splat(8.0) * t18 * t600 * t366 - f64x8::splat(9.0) / f64x8::splat(8.0) * t18 * t1262 * t149 - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t19 * t2627 * t49 - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t476 * t366 - f64x8::splat(3.0) / f64x8::splat(4.0) * t18 * t1015 * t149 + t18 * t1011 * t149 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t116 * t1261 * t49 + t18 * t289 * t599 * t49 / f64x8::splat(4.0) - f64x8::splat(5.0) / f64x8::splat(36.0) * t18 * t40 * t277 * t49;
            let t2654 = ((t2).select(f64x8::splat(0.0), t2653));
            let tv4rho3sigma0 = f64x8::splat(2.0) * v_rho * t2654 + f64x8::splat(6.0) * t1291;
            acc_v4rho3sigma = tv4rho3sigma0;
            let t2676 = f64x8::splat(4864.0) * t632 * t345 * t45 - f64x8::splat(629760.0) * param_b * t351 * t1412 + f64x8::splat(10838016.0) * param_b * t359 * t361 * t27 * t36 - f64x8::splat(142000128.0) * t140 * t1149 + f64x8::splat(637009920.0) * t356 / t837 / t30 * t1797;
            let t2719 = t161 * t516;
            let t2801 = -f64x8::splat(405.0) * t1080 * t2247 * t623 * t322 + f64x8::splat(135.0) / f64x8::splat(4.0) * t1080 * t1081 * t644 * t322 + f64x8::splat(135.0) / f64x8::splat(4.0) * t1080 * t1081 * t623 * t366 + f64x8::splat(3.0) / f64x8::splat(2.0) * t127 * t329 * t1367 * t366 - t127 * t329 * t494 * t1152 + f64x8::splat(9.0) / f64x8::splat(4.0) * t317 * t483 * t320 * t1424 * t149 + f64x8::splat(9.0) / f64x8::splat(8.0) * t317 * t483 * t1393 * t366 + f64x8::splat(945.0) / f64x8::splat(8.0) * t1951 * t1953 * t623 * t322 * t329 - f64x8::splat(27.0) / f64x8::splat(4.0) * t317 * t483 * t1372 * t366 - f64x8::splat(27.0) / f64x8::splat(4.0) * t317 * t483 * t781 * t644 * t322 + t793 * t329 * t1398 * t371 / f64x8::splat(9.0) + f64x8::splat(3.0) / f64x8::splat(2.0) * t127 * t329 * t1393 * t322 - f64x8::splat(6.0) * t127 * t329 * t1372 * t322 + f64x8::splat(81.0) / f64x8::splat(2.0) * t317 * t483 * t767 * t623 * t322 - f64x8::splat(2.0) / f64x8::splat(9.0) * t793 * t329 * t1381 * t371 + t1984 * t483 * t1367 * t371 / f64x8::splat(2.0);
            let t2819 = t1363 * t11;
            let t2823 = t1091 * t510;
            let t2828 = t644 * t11 * t149;
            let t2844 = t235 * t11 * t510;
            let t2854 = t510 * t510;
            let t2866 = -t328 * t329 * t130 * t1424 * t11 / f64x8::splat(6.0) - t127 * t329 * t336 * t1424 * t149 - t127 * t329 * t1403 * t366 / f64x8::splat(2.0) + f64x8::splat(9.0) / f64x8::splat(4.0) * t317 * t483 * t484 * t1152 - f64x8::splat(45.0) / f64x8::splat(2.0) * t1997 * t1081 * t2819 + f64x8::splat(135.0) * t1080 * t1081 * t2823 + t1117 * t1118 * t2828 / f64x8::splat(3.0) + f64x8::splat(6.0) * t2367 * t2352 * t2823 - f64x8::splat(3.0) / f64x8::splat(4.0) * t1089 * t1090 * t2828 - f64x8::splat(27.0) * t2347 * t2348 * t2823 - t1117 * t2352 * t2819 + f64x8::splat(2.0) / f64x8::splat(3.0) * t1117 * t1118 * t2844 + f64x8::splat(9.0) / f64x8::splat(2.0) * t1089 * t2348 * t2819 - f64x8::splat(3.0) / f64x8::splat(2.0) * t1089 * t1090 * t2844 + f64x8::splat(9.0) / f64x8::splat(4.0) * t317 * t318 * t321 * t2854 - t127 * t128 * t337 * t2854 + t127 * t128 * t131 * t2676 / f64x8::splat(4.0);
            let t2867 = t2801 + t2866;
            let t2868 = ((t63).select(t2867, f64x8::splat(0.0)));
            let t2883 = t516 * t516;
            let t2892 = -t247 * t2719 / f64x8::splat(20.0) + t245 * t2719 / f64x8::splat(2.0) - f64x8::splat(8.0) / f64x8::splat(3.0) * t243 * t2719 - t251 * t2719 / f64x8::splat(5376.0) + t249 * t2719 / f64x8::splat(288.0) + t1028 * t2719 / f64x8::splat(103219200.0) - t255 * t2719 / f64x8::splat(3317760.0) - t122 * t2868 / f64x8::splat(18.0) + t165 * t2868 / f64x8::splat(240.0) - t169 * t2868 / f64x8::splat(4480.0) + t173 * t2868 / f64x8::splat(103680.0) - t177 * t2868 / f64x8::splat(2838528.0) + t181 * t2868 / f64x8::splat(89456640.0) - t185 * t2868 / f64x8::splat(3185049600.0) + t75 * t2883 / f64x8::splat(320.0) - t72 * t2883 / f64x8::splat(24.0) + t69 * t2883 / f64x8::splat(3.0) + t81 * t2883 / f64x8::splat(129024.0);
            let t2934 = -t78 * t2883 / f64x8::splat(5760.0) - t408 * t2883 / f64x8::splat(3715891200.0) + t87 * t2883 / f64x8::splat(106168320.0) - t84 * t2883 / f64x8::splat(3440640.0) + t189 * t2868 / f64x8::splat(126340300800.0) + t81 * t1430 * t161 / f64x8::splat(129024.0) - t84 * t1430 * t161 / f64x8::splat(3440640.0) + t87 * t1430 * t161 / f64x8::splat(106168320.0) - t408 * t1430 * t161 / f64x8::splat(3715891200.0) + t75 * t1430 * t161 / f64x8::splat(320.0) + t1357 * t380 / f64x8::splat(640.0) - f64x8::splat(2.0) / f64x8::splat(3.0) * t1360 * t380 + f64x8::splat(10.0) / f64x8::splat(3.0) * t653 * t301 - t663 * t301 / f64x8::splat(80.0) + t659 * t301 / f64x8::splat(8.0) + f64x8::splat(9.0) / f64x8::splat(80.0) * t661 * t301 - f64x8::splat(7.0) / f64x8::splat(8.0) * t657 * t301 - f64x8::splat(2.0) / f64x8::splat(3.0) * t655 * t301;
            let t2976 = -t671 * t301 / f64x8::splat(21504.0) + f64x8::splat(13.0) / f64x8::splat(21504.0) * t669 * t301 + t667 * t301 / f64x8::splat(1152.0) - f64x8::splat(11.0) / f64x8::splat(1152.0) * t665 * t301 + t878 * t650 * t301 / f64x8::splat(412876800.0) - t679 * t301 / f64x8::splat(13271040.0) - f64x8::splat(19.0) / f64x8::splat(412876800.0) * t1890 * t620 * t301 + f64x8::splat(17.0) / f64x8::splat(13271040.0) * t677 * t301 + t675 * t301 / f64x8::splat(491520.0) - t673 * t301 / f64x8::splat(32768.0) - t1349 * t380 / f64x8::splat(48.0) - t1352 * t380 / f64x8::splat(80.0) + t524 * t1158 / f64x8::splat(320.0) + t1344 * t380 / f64x8::splat(8.0) - t519 * t1158 / f64x8::splat(24.0) - t72 * t1430 * t161 / f64x8::splat(24.0) + t480 * t1158 / f64x8::splat(3.0) + t69 * t1430 * t161 / f64x8::splat(3.0);
            let t3014 = t1341 * t380 / f64x8::splat(6.0) + t534 * t1158 / f64x8::splat(129024.0) + t1335 * t380 / f64x8::splat(258048.0) - t1327 * t380 / f64x8::splat(11520.0) - t1330 * t380 / f64x8::splat(21504.0) - t1319 * t380 / f64x8::splat(7431782400.0) + t1322 * t380 / f64x8::splat(1152.0) - t529 * t1158 / f64x8::splat(5760.0) + t1314 * t380 / f64x8::splat(412876800.0) - t549 * t1158 / f64x8::splat(3715891200.0) + t1311 * t380 / f64x8::splat(212336640.0) - t1303 * t380 / f64x8::splat(6881280.0) - t1306 * t380 / f64x8::splat(13271040.0) + t544 * t1158 / f64x8::splat(106168320.0) + t1298 * t380 / f64x8::splat(491520.0) - t539 * t1158 / f64x8::splat(3440640.0) - t78 * t1430 * t161 / f64x8::splat(5760.0) + t253 * t2719 / f64x8::splat(122880.0);
            let t3017 = ((t63).select(f64x8::splat(0.0), t2867));
            let t3040 = t555 * t555;
            let t3066 = t99 * t1449;
            let t3079 = -f64x8::splat(8.0) * t3040 * t100 - f64x8::splat(2.0) * t97 * t3040 * t99 - t92 * t3017 * t99 - f64x8::splat(4.0) * t204 * t3017 - f64x8::splat(4.0) * t1241 * t682 - f64x8::splat(8.0) * t583 * t1449 - f64x8::splat(8.0) * t1510 * t1177 + t200 * t3017 * t99 / f64x8::splat(2.0) + t442 * t3040 * t99 / f64x8::splat(2.0) - f64x8::splat(4.0) * t434 * t3040 * t99 - t1246 * t1472 - f64x8::splat(2.0) * t586 * t3066 - f64x8::splat(2.0) * t1513 * t2525 + t1507 * t934 / f64x8::splat(8.0) + t2066 * t687 * t1219 / f64x8::splat(16.0) + f64x8::splat(15.0) / f64x8::splat(2.0) * t421 * t682 * t1219;
            let t3084 = t973 * t682;
            let t3089 = t442 * t1449;
            let t3111 = t574 * t555;
            let t3118 = -f64x8::splat(75.0) / f64x8::splat(2.0) * t704 * t1219 + t1502 * t934 / f64x8::splat(4.0) + t3084 * t1219 / f64x8::splat(8.0) + t580 * t2525 / f64x8::splat(2.0) + t3089 * t574 / f64x8::splat(2.0) - f64x8::splat(4.0) * t434 * t1449 * t574 - f64x8::splat(2.0) * t1496 * t934 - f64x8::splat(5.0) / f64x8::splat(2.0) * t962 * t682 * t1219 - f64x8::splat(5.0) / f64x8::splat(2.0) * t1493 * t934 - f64x8::splat(19.0) / f64x8::splat(8.0) * t2079 * t687 * t1219 + f64x8::splat(15.0) / f64x8::splat(2.0) * t688 * t934 + f64x8::splat(85.0) / f64x8::splat(4.0) * t1463 * t1219 - f64x8::splat(4.0) * t573 * t2525 + t1238 * t3111 / f64x8::splat(2.0) - f64x8::splat(10.0) * t1225 * t3111 + f64x8::splat(30.0) * t1218 * t3111;
            let t3145 = f64x8::splat(2.0) * t682 * t454 + f64x8::splat(4.0) * t1449 * t209 + f64x8::splat(2.0) * t3017 * t103 + f64x8::splat(2.0) * t90 * (t3079 + t3118) + f64x8::splat(2.0) * t414 * t715 + f64x8::splat(4.0) * t193 * t1523 + f64x8::splat(4.0) * t258 * t1254 + f64x8::splat(8.0) * t555 * t592 + f64x8::splat(4.0) * t1177 * t270 - t195 * t3017 + f64x8::splat(4.0) * t426 * t3040 - t421 * t3040 * t99 + f64x8::splat(4.0) * t426 * t258 * t1177 - t1463 * t934 / f64x8::splat(4.0) - t2116 * t687 * t1219 / f64x8::splat(8.0);
            let t3181 = f64x8::splat(7.0) / f64x8::splat(2.0) * t704 * t934 + f64x8::splat(15.0) / f64x8::splat(4.0) * t1507 * t1219 - t1218 * t2525 - f64x8::splat(6.0) * t942 * t682 * t422 + f64x8::splat(7.0) / f64x8::splat(2.0) * t443 * t1472 + f64x8::splat(24.0) * t2123 * t687 * t422 - f64x8::splat(24.0) * t1493 * t1219 + f64x8::splat(2.0) * t426 * t682 * t414 - t1190 * t1472 / f64x8::splat(2.0) - t559 * t3066 - t1193 * t1472 / f64x8::splat(4.0) + f64x8::splat(4.0) * t426 * t1449 * t193 - f64x8::splat(6.0) * t942 * t687 * t414 - f64x8::splat(24.0) * t942 * t563 * t555 - t2583 * t3111 + f64x8::splat(14.0) * t580 * t3111;
            let t3186 = ((t62).select(t2892 + t2934 + t2976 + t3014, -f64x8::splat(8.0) / f64x8::splat(3.0) * t3017 * t106 - f64x8::splat(16.0) / f64x8::splat(3.0) * t1449 * t212 - f64x8::splat(8.0) / f64x8::splat(3.0) * t682 * t457 - f64x8::splat(16.0) / f64x8::splat(3.0) * t1177 * t273 - f64x8::splat(32.0) / f64x8::splat(3.0) * t555 * t595 - f64x8::splat(16.0) / f64x8::splat(3.0) * t258 * t1257 - f64x8::splat(8.0) / f64x8::splat(3.0) * t414 * t718 - f64x8::splat(16.0) / f64x8::splat(3.0) * t193 * t1526 - f64x8::splat(8.0) / f64x8::splat(3.0) * t90 * (t3145 + t3181)));
            let t3202 = -f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t111 * t2676 - f64x8::splat(3.0) / f64x8::splat(4.0) * t18 * t217 * t1424 - t18 * t117 * t1424 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t462 * t644 - f64x8::splat(3.0) / f64x8::splat(4.0) * t18 * t278 * t1152 - t18 * t294 * t644 / f64x8::splat(4.0) + t18 * t290 * t644 / f64x8::splat(12.0) - f64x8::splat(3.0) / f64x8::splat(2.0) * t18 * t600 * t510 - t18 * t476 * t510 / f64x8::splat(2.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t18 * t1262 * t235 - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t723 * t366 - t18 * t1015 * t235 / f64x8::splat(2.0) + t18 * t1011 * t235 / f64x8::splat(6.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t18 * t1531 * t149 - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t19 * t3186 * t49 - t18 * t1294 * t149 / f64x8::splat(4.0) - t18 * t116 * t1530 * t49 / f64x8::splat(4.0) + t18 * t289 * t722 * t49 / f64x8::splat(12.0);
            let t3203 = ((t2).select(f64x8::splat(0.0), t3202));
            let tv4rho2sigma20 = f64x8::splat(2.0) * v_rho * t3203 + f64x8::splat(4.0) * t1557;
            acc_v4rho2sigma2 = tv4rho2sigma20;
            let t3210 = t781 * t1594;
            let t3260 = -f64x8::splat(6.0) * t127 * t329 * t3210 * t149 + f64x8::splat(9.0) / f64x8::splat(2.0) * t127 * t329 * t1367 * t510 - f64x8::splat(3.0) / f64x8::splat(2.0) * t127 * t329 * t1131 * t644 - f64x8::splat(3.0) / f64x8::splat(2.0) * t127 * t329 * t494 * t1424 - t328 * t329 * t130 * t1626 * t11 / f64x8::splat(12.0) - t127 * t329 * t336 * t1626 * t149 / f64x8::splat(2.0) + f64x8::splat(9.0) / f64x8::splat(8.0) * t317 * t483 * t320 * t1626 * t149 + f64x8::splat(945.0) / f64x8::splat(8.0) * t1951 * t1953 * t1594 * t20 * t26 * t149 + f64x8::splat(9.0) / f64x8::splat(4.0) * t774 * t483 * t3210 * t11 + f64x8::splat(81.0) / f64x8::splat(2.0) * t317 * t483 * t1595 * t149 - f64x8::splat(81.0) / f64x8::splat(4.0) * t317 * t483 * t1372 * t510;
            let t3274 = t235 * t644;
            let t3275 = t3274 * t11;
            let t3279 = t3274 * t149;
            let t3302 = f64x8::splat(82944.0) * t1411 * t145 - f64x8::splat(2488320.0) * param_b * t505 * t361 * v_sigma * t36 + f64x8::splat(43794432.0) * t28 * t1421 - f64x8::splat(238878720.0) * t140 / t1785 * t1797;
            let t3322 = f64x8::splat(27.0) / f64x8::splat(8.0) * t317 * t483 * t1101 * t644 + f64x8::splat(27.0) / f64x8::splat(8.0) * t317 * t483 * t484 * t1424 - t328 * t329 * t320 * t1594 * t11 / f64x8::splat(2.0) - f64x8::splat(9.0) / f64x8::splat(8.0) * t1089 * t1090 * t3275 - f64x8::splat(81.0) / f64x8::splat(4.0) * t2347 * t2348 * t3279 + t1117 * t1118 * t3275 / f64x8::splat(2.0) + f64x8::splat(9.0) / f64x8::splat(2.0) * t2367 * t2352 * t3279 + t127 * t128 * t131 * t3302 / f64x8::splat(4.0) + f64x8::splat(405.0) / f64x8::splat(4.0) * t1080 * t1081 * t3279 - f64x8::splat(45.0) / f64x8::splat(4.0) * t1997 * t1081 * t1594 * t11 - f64x8::splat(405.0) * t1080 * t2247 * t1594 * t149 + f64x8::splat(405.0) / f64x8::splat(4.0) * t1080 * t1081 * t623 * t510;
            let t3323 = t3260 + t3322;
            let t3324 = ((t63).select(t3323, f64x8::splat(0.0)));
            let t3355 = t189 * t3324 / f64x8::splat(126340300800.0) - t185 * t3324 / f64x8::splat(3185049600.0) - t122 * t3324 / f64x8::splat(18.0) + t165 * t3324 / f64x8::splat(240.0) - t169 * t3324 / f64x8::splat(4480.0) + t173 * t3324 / f64x8::splat(103680.0) - t177 * t3324 / f64x8::splat(2838528.0) + t181 * t3324 / f64x8::splat(89456640.0) - t87 * t1560 * t161 / f64x8::splat(32768.0) + t1298 * t516 / f64x8::splat(163840.0) - t1075 * t650 / f64x8::splat(2293760.0) - t539 * t1430 / f64x8::splat(2293760.0) + f64x8::splat(17.0) / f64x8::splat(13271040.0) * t408 * t1560 * t161 - t1306 * t516 / f64x8::splat(4423680.0);
            let t3387 = t1065 * t650 / f64x8::splat(70778880.0) + t544 * t1430 / f64x8::splat(70778880.0) - f64x8::splat(19.0) / f64x8::splat(412876800.0) * t1890 * t1560 * t161 + t1314 * t516 / f64x8::splat(137625600.0) - t1070 * t650 / f64x8::splat(2477260800.0) - t549 * t1430 / f64x8::splat(2477260800.0) + f64x8::splat(3.0) / f64x8::splat(640.0) * t1056 * t650 + f64x8::splat(3.0) / f64x8::splat(640.0) * t524 * t1430 - f64x8::splat(11.0) / f64x8::splat(1152.0) * t81 * t1560 * t161 + t1322 * t516 / f64x8::splat(384.0) - t1046 * t650 / f64x8::splat(3840.0) - t529 * t1430 / f64x8::splat(3840.0) + f64x8::splat(13.0) / f64x8::splat(21504.0) * t84 * t1560 * t161 - t1330 * t516 / f64x8::splat(7168.0);
            let t3422 = t1041 * t650 / f64x8::splat(86016.0) + t534 * t1430 / f64x8::splat(86016.0) + f64x8::splat(10.0) / f64x8::splat(3.0) * t72 * t1560 * t161 - f64x8::splat(2.0) * t1360 * t516 + t1062 * t650 / f64x8::splat(2.0) + t480 * t1430 / f64x8::splat(2.0) - f64x8::splat(7.0) / f64x8::splat(8.0) * t75 * t1560 * t161 + f64x8::splat(3.0) / f64x8::splat(8.0) * t1344 * t516 - t1051 * t650 / f64x8::splat(16.0) - t519 * t1430 / f64x8::splat(16.0) + f64x8::splat(9.0) / f64x8::splat(80.0) * t78 * t1560 * t161 - f64x8::splat(3.0) / f64x8::splat(80.0) * t1352 * t516 + t69 * t1632 * t161 / f64x8::splat(6.0) - t72 * t1632 * t161 / f64x8::splat(48.0);
            let t3441 = t650 * t161;
            let t3458 = t75 * t1632 * t161 / f64x8::splat(640.0) - t78 * t1632 * t161 / f64x8::splat(11520.0) + t81 * t1632 * t161 / f64x8::splat(258048.0) - t84 * t1632 * t161 / f64x8::splat(6881280.0) + t87 * t1632 * t161 / f64x8::splat(212336640.0) - t408 * t1632 * t161 / f64x8::splat(7431782400.0) + f64x8::splat(3.0) / f64x8::splat(8.0) * t245 * t3441 - f64x8::splat(2.0) * t243 * t3441 - t251 * t3441 / f64x8::splat(7168.0) + t249 * t3441 / f64x8::splat(384.0) + t1028 * t3441 / f64x8::splat(137625600.0) - f64x8::splat(3.0) / f64x8::splat(80.0) * t247 * t3441 - t255 * t3441 / f64x8::splat(4423680.0) + t253 * t3441 / f64x8::splat(163840.0);
            let t3461 = ((t63).select(f64x8::splat(0.0), t3323));
            let t3476 = t99 * t1651;
            let t3507 = -t559 * t3476 / f64x8::splat(2.0) + f64x8::splat(6.0) * t426 * t555 * t682 + f64x8::splat(6.0) * t426 * t258 * t1449 + f64x8::splat(2.0) * t426 * t1651 * t193 - f64x8::splat(3.0) / f64x8::splat(2.0) * t1218 * t3066 - f64x8::splat(3.0) / f64x8::splat(4.0) * t1463 * t1196 - t2116 * t1658 * t574 / f64x8::splat(8.0) + f64x8::splat(24.0) * t2123 * t1658 * t193 - f64x8::splat(18.0) * t942 * t687 * t555 - f64x8::splat(24.0) * t1684 * t574 + f64x8::splat(21.0) / f64x8::splat(2.0) * t704 * t1196 + f64x8::splat(15.0) / f64x8::splat(4.0) * t1692 * t574;
            let t3536 = -f64x8::splat(15.0) / f64x8::splat(2.0) * t1493 * t1196 - f64x8::splat(19.0) / f64x8::splat(8.0) * t2079 * t1658 * t574 - f64x8::splat(2.0) * t434 * t1651 * t574 - f64x8::splat(75.0) / f64x8::splat(2.0) * t1659 * t574 + f64x8::splat(45.0) / f64x8::splat(2.0) * t688 * t1196 + f64x8::splat(85.0) / f64x8::splat(4.0) * t1664 * t574 - f64x8::splat(6.0) * t1222 * t1472 - t586 * t3476 - f64x8::splat(3.0) * t97 * t555 * t1472 - f64x8::splat(3.0) * t1513 * t3066 + f64x8::splat(3.0) / f64x8::splat(4.0) * t1502 * t1196 + f64x8::splat(3.0) / f64x8::splat(8.0) * t1507 * t1196;
            let t3540 = t442 * t1651;
            let t3563 = t574 * t682;
            let t3568 = t2066 * t1658 * t574 / f64x8::splat(16.0) + t3540 * t574 / f64x8::splat(4.0) + f64x8::splat(3.0) / f64x8::splat(4.0) * t3089 * t560 - f64x8::splat(6.0) * t573 * t3066 - t92 * t3461 * t99 - f64x8::splat(4.0) * t583 * t1651 - f64x8::splat(4.0) * t204 * t3461 - f64x8::splat(12.0) * t1510 * t1449 - f64x8::splat(12.0) * t555 * t100 * t682 + t200 * t3461 * t99 / f64x8::splat(2.0) + f64x8::splat(3.0) / f64x8::splat(8.0) * t3084 * t2498 - f64x8::splat(15.0) / f64x8::splat(2.0) * t1225 * t3563 + f64x8::splat(45.0) / f64x8::splat(2.0) * t1218 * t3563;
            let t3594 = -f64x8::splat(3.0) / f64x8::splat(2.0) * t2580 * t1472 + f64x8::splat(2.0) * t90 * (t3536 + t3568) + f64x8::splat(2.0) * t193 * t1703 + f64x8::splat(6.0) * t258 * t1523 + f64x8::splat(6.0) * t555 * t715 + f64x8::splat(6.0) * t682 * t592 + f64x8::splat(6.0) * t1449 * t270 + f64x8::splat(2.0) * t1651 * t209 + f64x8::splat(2.0) * t3461 * t103 - t195 * t3461 + f64x8::splat(21.0) / f64x8::splat(2.0) * t580 * t3563 - f64x8::splat(18.0) * t942 * t1669 * t193 - f64x8::splat(3.0) / f64x8::splat(4.0) * t2583 * t3563;
            let t3599 = ((t62).select(t3355 + t3387 + t3422 + t3458, -f64x8::splat(8.0) / f64x8::splat(3.0) * t3461 * t106 - f64x8::splat(8.0) / f64x8::splat(3.0) * t1651 * t212 - f64x8::splat(8.0) * t1449 * t273 - f64x8::splat(8.0) * t682 * t595 - f64x8::splat(8.0) * t555 * t718 - f64x8::splat(8.0) * t258 * t1526 - f64x8::splat(8.0) / f64x8::splat(3.0) * t193 * t1706 - f64x8::splat(8.0) / f64x8::splat(3.0) * t90 * (t3507 + t3594)));
            let t3634 = -t18 * t116 * t1710 * t49 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t19 * t3599 * t49 - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t1711 * t149 - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t1294 * t235 - f64x8::splat(9.0) / f64x8::splat(8.0) * t18 * t1531 * t235 - f64x8::splat(9.0) / f64x8::splat(8.0) * t18 * t723 * t510 - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t476 * t644 - f64x8::splat(9.0) / f64x8::splat(8.0) * t18 * t600 * t644 - f64x8::splat(9.0) / f64x8::splat(8.0) * t18 * t278 * t1424 - t18 * t117 * t1626 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t217 * t1626 - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t111 * t3302;
            let t3635 = ((t2).select(f64x8::splat(0.0), t3634));
            let tv4rhosigma30 = f64x8::splat(2.0) * v_rho * t3635 + f64x8::splat(2.0) * t1725;
            acc_v4rhosigma3 = tv4rhosigma30;
            let t3638 = t623 * t623;
            let t3659 = t644 * t644;
            let t3694 = f64x8::splat(373248.0) * param_b * t639 * t361 * t36 - f64x8::splat(11943936.0) * t228 * t1623 + f64x8::splat(89579520.0) * t28 / t837 * t1797;
            let t3699 = f64x8::splat(945.0) / f64x8::splat(8.0) * t1951 * t1953 * t3638 * t329 - f64x8::splat(405.0) * t764 * t765 * t2003 * t3638 + f64x8::splat(405.0) / f64x8::splat(2.0) * t1080 * t1081 * t623 * t644 + f64x8::splat(81.0) / f64x8::splat(2.0) * t317 * t318 * t1917 * t3638 - f64x8::splat(81.0) / f64x8::splat(2.0) * t317 * t483 * t1372 * t644 + f64x8::splat(27.0) / f64x8::splat(8.0) * t317 * t318 * t321 * t3659 + f64x8::splat(9.0) / f64x8::splat(2.0) * t317 * t483 * t484 * t1626 - f64x8::splat(6.0) * t127 * t128 * t1907 * t3638 + f64x8::splat(9.0) * t127 * t329 * t1367 * t644 - f64x8::splat(3.0) / f64x8::splat(2.0) * t127 * t128 * t337 * t3659 - f64x8::splat(2.0) * t127 * t329 * t494 * t1626 + t127 * t128 * t131 * t3694 / f64x8::splat(4.0);
            let t3700 = ((t63).select(t3699, f64x8::splat(0.0)));
            let t3717 = t650 * t650;
            let t3720 = t620 * t620;
            let t3743 = -t169 * t3700 / f64x8::splat(4480.0) - t122 * t3700 / f64x8::splat(18.0) + t165 * t3700 / f64x8::splat(240.0) - t185 * t3700 / f64x8::splat(3185049600.0) + t189 * t3700 / f64x8::splat(126340300800.0) + t181 * t3700 / f64x8::splat(89456640.0) + t173 * t3700 / f64x8::splat(103680.0) - t177 * t3700 / f64x8::splat(2838528.0) + t69 * t3717 / f64x8::splat(2.0) - f64x8::splat(7.0) / f64x8::splat(8.0) * t75 * t3720 - t72 * t3717 / f64x8::splat(16.0) + f64x8::splat(9.0) / f64x8::splat(80.0) * t78 * t3720 - f64x8::splat(11.0) / f64x8::splat(1152.0) * t81 * t3720 - t78 * t3717 / f64x8::splat(3840.0) + f64x8::splat(13.0) / f64x8::splat(21504.0) * t84 * t3720 + t81 * t3717 / f64x8::splat(86016.0) + f64x8::splat(10.0) / f64x8::splat(3.0) * t72 * t3720 - f64x8::splat(19.0) / f64x8::splat(412876800.0) * t1890 * t3720 - t408 * t3717 / f64x8::splat(2477260800.0) + f64x8::splat(3.0) / f64x8::splat(640.0) * t75 * t3717;
            let t3784 = -t87 * t3720 / f64x8::splat(32768.0) - t84 * t3717 / f64x8::splat(2293760.0) + f64x8::splat(17.0) / f64x8::splat(13271040.0) * t408 * t3720 + t87 * t3717 / f64x8::splat(70778880.0) + f64x8::splat(2.0) / f64x8::splat(3.0) * t480 * t1632 + f64x8::splat(3.0) / f64x8::splat(4.0) * t1344 * t650 - t519 * t1632 / f64x8::splat(12.0) + t1322 * t650 / f64x8::splat(192.0) - t529 * t1632 / f64x8::splat(2880.0) - t1330 * t650 / f64x8::splat(3584.0) + t534 * t1632 / f64x8::splat(64512.0) - f64x8::splat(4.0) * t1360 * t650 + t544 * t1632 / f64x8::splat(53084160.0) + t1314 * t650 / f64x8::splat(68812800.0) - t549 * t1632 / f64x8::splat(1857945600.0) + t524 * t1632 / f64x8::splat(160.0) + t1298 * t650 / f64x8::splat(81920.0) - t539 * t1632 / f64x8::splat(1720320.0) - t1306 * t650 / f64x8::splat(2211840.0) - f64x8::splat(3.0) / f64x8::splat(40.0) * t1352 * t650;
            let t3786 = ((t63).select(f64x8::splat(0.0), t3699));
            let t3795 = t687 * t687;
            let t3809 = t682 * t682;
            let t3866 = -f64x8::splat(4.0) * t204 * t3786 - t92 * t3786 * t99 + t2066 * t3795 * t99 / f64x8::splat(16.0) - f64x8::splat(12.0) * t3809 * t100 - f64x8::splat(16.0) * t1510 * t1651 - f64x8::splat(19.0) / f64x8::splat(8.0) * t2079 * t3795 * t99 + t200 * t3786 * t99 / f64x8::splat(2.0) + f64x8::splat(85.0) / f64x8::splat(4.0) * t938 * t3795 * t99 - f64x8::splat(75.0) / f64x8::splat(2.0) * t442 * t3795 * t99 + f64x8::splat(45.0) * t688 * t1472 - f64x8::splat(6.0) * t434 * t3809 * t99 - f64x8::splat(8.0) * t573 * t3476 - f64x8::splat(15.0) * t1493 * t1472 + t3540 * t560 + f64x8::splat(3.0) / f64x8::splat(4.0) * t442 * t3809 * t99 + f64x8::splat(3.0) / f64x8::splat(4.0) * t3084 * t687 * t99 - f64x8::splat(3.0) * t97 * t3809 * t99 - f64x8::splat(4.0) * t1513 * t3476;
            let t3881 = f64x8::splat(15.0) / f64x8::splat(4.0) * t973 * t3795 * t99 + f64x8::splat(8.0) * t426 * t258 * t1651 - f64x8::splat(3.0) / f64x8::splat(2.0) * t1463 * t1472 + f64x8::splat(24.0) * t2123 * t3795 - f64x8::splat(36.0) * t942 * t687 * t682 + f64x8::splat(6.0) * t426 * t3809 - f64x8::splat(24.0) * t962 * t3795 * t99 + f64x8::splat(21.0) * t704 * t1472 - f64x8::splat(3.0) / f64x8::splat(2.0) * t421 * t3809 * t99 - f64x8::splat(2.0) * t1218 * t3476 + f64x8::splat(2.0) * t90 * t3866 + f64x8::splat(12.0) * t682 * t715 + f64x8::splat(8.0) * t258 * t1703 + f64x8::splat(8.0) * t1651 * t270 + f64x8::splat(2.0) * t3786 * t103 - t2116 * t3795 * t99 / f64x8::splat(8.0) - t195 * t3786;
            let t3885 = ((t62).select(t3743 + t3784, -f64x8::splat(8.0) / f64x8::splat(3.0) * t3786 * t106 - f64x8::splat(32.0) / f64x8::splat(3.0) * t1651 * t273 - f64x8::splat(16.0) * t682 * t718 - f64x8::splat(32.0) / f64x8::splat(3.0) * t258 * t1706 - f64x8::splat(8.0) / f64x8::splat(3.0) * t90 * t3881));
            let t3903 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t19 * t3885 * t49 - f64x8::splat(3.0) / f64x8::splat(2.0) * t18 * t1711 * t235 - f64x8::splat(9.0) / f64x8::splat(4.0) * t18 * t723 * t644 - f64x8::splat(3.0) / f64x8::splat(2.0) * t18 * t278 * t1626 - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t111 * t3694));
            let tv4sigma40 = f64x8::splat(2.0) * v_rho * t3903;
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
