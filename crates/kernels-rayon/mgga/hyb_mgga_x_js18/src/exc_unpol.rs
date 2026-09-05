//! HYB_MGGA_X_JS18 exc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/hyb_mgga_x_js18.c`
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
pub fn hyb_mgga_x_js18_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_hyb_omega_0: f64,
    param_hyb_coeff_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_hyb_omega_0 = f64x8::splat(param_hyb_omega_0);
    let param_hyb_coeff_0 = f64x8::splat(param_hyb_coeff_0);
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
        {
            let t3 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t4 = f64x8::splat(M_CBRT3);
            let t5 = f64x8::splat(M_CBRTPI);
            let t7 = t4 / t5;
            let t8 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t9 = zeta_threshold - f64x8::splat(1.0);
            let t11 = ((t8).select(t9, (t8).select(-t9, f64x8::splat(0.0))));
            let t12 = f64x8::splat(1.0) + t11;
            let t13 = (t12).simd_le(zeta_threshold);
            let t14 = (simd::cbrt(zeta_threshold));
            let t16 = (simd::cbrt(t12));
            let t18 = ((t13).select(t14 * zeta_threshold, t16 * t12));
            let t19 = (simd::cbrt(v_rho));
            let t20 = t18 * t19;
            let t21 = f64x8::splat(1.0) / v_rho;
            let t22 = v_sigma * t21;
            let t23 = f64x8::splat(1.0) / v_tau;
            let t25 = t22 * t23 / f64x8::splat(8.0);
            let t26 = (t25).simd_lt(f64x8::splat(1.0));
            let t27 = ((t26).select(t25, f64x8::splat(1.0)));
            let t28 = t27 * t27;
            let t29 = t28 * t27;
            let t31 = t28 + f64x8::splat(3.0) * t29;
            let t32 = f64x8::splat(1.0) + t29;
            let t33 = t32 * t32;
            let t34 = f64x8::splat(1.0) / t33;
            let t35 = t31 * t34;
            let t36 = (simd::cbrt(f64x8::splat(9.0)));
            let t37 = t36 * t36;
            let t39 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t40 = t39 * t39;
            let t41 = t37 * t40;
            let t42 = t41 * param_hyb_omega_0;
            let t43 = f64x8::splat(1.0) / t19;
            let t44 = t4 * t43;
            let t45 = f64x8::splat(M_CBRT6);
            let t46 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t47 = (simd::cbrt(t46));
            let t48 = t47 * t47;
            let t49 = f64x8::splat(1.0) / t48;
            let t50 = t45 * t49;
            let t51 = f64x8::splat(M_CBRT2);
            let t52 = t51 * t51;
            let t53 = v_sigma * t52;
            let t54 = v_rho * v_rho;
            let t55 = t19 * t19;
            let t57 = f64x8::splat(1.0) / t55 / t54;
            let t58 = t53 * t57;
            let t59 = t50 * t58;
            let t61 = t45 * t45;
            let t63 = f64x8::splat(1.0) / t47 / t46;
            let t64 = t61 * t63;
            let t65 = v_sigma * v_sigma;
            let t66 = t65 * t51;
            let t67 = t54 * t54;
            let t68 = t67 * v_rho;
            let t70 = f64x8::splat(1.0) / t19 / t68;
            let t74 = f64x8::splat(1.0) + f64x8::splat(0.1504548888888889) * t59 + f64x8::splat(0.00537989809245259) * t64 * t66 * t70;
            let t75 = (simd::pow(t74, f64x8::splat(1.0) / f64x8::splat(10.0)));
            let t77 = ((t13).select(t14, t16));
            let t78 = f64x8::splat(1.0) / t77;
            let t79 = f64x8::splat(1.0) / t75 * t78;
            let t82 = t42 * t44 * t79 / f64x8::splat(18.0);
            let t83 = (t82).simd_lt(f64x8::splat(1e-10));
            let t84 = ((t83).select(f64x8::splat(1e-10), t82));
            let t85 = (f64x8::splat(1.35)).simd_le(t84);
            let t86 = (f64x8::splat(1.35)).simd_lt(t84);
            let t87 = ((t86).select(t84, f64x8::splat(1.35)));
            let t88 = t87 * t87;
            let t91 = t88 * t88;
            let t92 = f64x8::splat(1.0) / t91;
            let t94 = t91 * t88;
            let t95 = f64x8::splat(1.0) / t94;
            let t97 = t91 * t91;
            let t98 = f64x8::splat(1.0) / t97;
            let t101 = f64x8::splat(1.0) / t97 / t88;
            let t104 = f64x8::splat(1.0) / t97 / t91;
            let t107 = f64x8::splat(1.0) / t97 / t94;
            let t109 = t97 * t97;
            let t110 = f64x8::splat(1.0) / t109;
            let t113 = ((t86).select(f64x8::splat(1.35), t84));
            let t114 = ((f64x8::splat(M_PI)).sqrt());
            let t115 = f64x8::splat(1.0) / t113;
            let t117 = (simd::erf(t115 / f64x8::splat(2.0)));
            let t119 = t113 * t113;
            let t120 = f64x8::splat(1.0) / t119;
            let t122 = (simd::exp(-t120 / f64x8::splat(4.0)));
            let t123 = t122 - f64x8::splat(1.0);
            let t126 = t122 - f64x8::splat(3.0) / f64x8::splat(2.0) - f64x8::splat(2.0) * t119 * t123;
            let t129 = f64x8::splat(2.0) * t113 * t126 + t114 * t117;
            let t133 = ((t85).select(f64x8::splat(1.0) / t88 / f64x8::splat(36.0) - t92 / f64x8::splat(960.0) + t95 / f64x8::splat(26880.0) - t98 / f64x8::splat(829440.0) + t101 / f64x8::splat(28385280.0) - t104 / f64x8::splat(1073479680.0) + t107 / f64x8::splat(44590694400.0) - t110 / f64x8::splat(2021444812800.0), f64x8::splat(1.0) - f64x8::splat(8.0) / f64x8::splat(3.0) * t113 * t129));
            let t134 = (simd::pow(t74, f64x8::splat(1.0) / f64x8::splat(5.0)));
            let t135 = f64x8::splat(1.0) / t134;
            let t137 = (f64x8::splat(0.27)).simd_le(t84);
            let t138 = (f64x8::splat(0.27)).simd_lt(t84);
            let t139 = ((t138).select(t84, f64x8::splat(0.27)));
            let t140 = t139 * t139;
            let t141 = t140 * t140;
            let t142 = t141 * t141;
            let t143 = t142 * t141;
            let t144 = t142 * t142;
            let t145 = t144 * t144;
            let t147 = f64x8::splat(1.0) / t145 / t143;
            let t149 = t141 * t140;
            let t150 = t142 * t149;
            let t152 = f64x8::splat(1.0) / t145 / t150;
            let t156 = f64x8::splat(1.0) / t149;
            let t158 = f64x8::splat(1.0) / t142;
            let t160 = t142 * t140;
            let t161 = f64x8::splat(1.0) / t160;
            let t163 = f64x8::splat(1.0) / t143;
            let t165 = f64x8::splat(1.0) / t150;
            let t167 = f64x8::splat(1.0) / t144;
            let t169 = t144 * t140;
            let t170 = f64x8::splat(1.0) / t169;
            let t173 = f64x8::splat(1.0) / t144 / t141;
            let t175 = t147 / f64x8::splat(3.3929038000650147e+37) - t152 / f64x8::splat(3.511556992918352e+39) + f64x8::splat(3.0) / f64x8::splat(2240.0) / t141 - t156 / f64x8::splat(11520.0) + f64x8::splat(3.0) / f64x8::splat(788480.0) * t158 - t161 / f64x8::splat(7454720.0) + t163 / f64x8::splat(247726080.0) - t165 / f64x8::splat(9358540800.0) + t167 / f64x8::splat(394474291200.0) - t170 / f64x8::splat(18311911833600.0) + t173 / f64x8::splat(927028425523200.0);
            let t177 = f64x8::splat(1.0) / t144 / t149;
            let t180 = f64x8::splat(1.0) / t144 / t142;
            let t183 = f64x8::splat(1.0) / t144 / t160;
            let t186 = f64x8::splat(1.0) / t144 / t143;
            let t189 = f64x8::splat(1.0) / t144 / t150;
            let t191 = f64x8::splat(1.0) / t145;
            let t194 = f64x8::splat(1.0) / t145 / t140;
            let t197 = f64x8::splat(1.0) / t145 / t141;
            let t200 = f64x8::splat(1.0) / t145 / t149;
            let t203 = f64x8::splat(1.0) / t145 / t142;
            let t206 = f64x8::splat(1.0) / t145 / t160;
            let t208 = -t177 / f64x8::splat(5.0785035485184e+16) + t180 / f64x8::splat(2.991700272218112e+18) - t183 / f64x8::splat(1.88514051721003e+20) + t186 / f64x8::splat(1.2648942844388573e+22) - t189 / f64x8::splat(9.002316741416457e+23) + t191 / f64x8::splat(6.772652029299977e+25) - t194 / f64x8::splat(5.36974553751641e+27) + t197 / f64x8::splat(4.474731034888079e+29) - t200 / f64x8::splat(3.909716563474291e+31) + t203 / f64x8::splat(3.5738523369945735e+33) - t206 / f64x8::splat(3.410951160703658e+35);
            let t210 = ((t138).select(f64x8::splat(0.27), t84));
            let t211 = t210 * t210;
            let t213 = t211 * t211;
            let t214 = f64x8::splat(64.0) * t213;
            let t215 = f64x8::splat(20.0) * t211 - t214;
            let t218 = (simd::exp(-f64x8::splat(1.0) / t211 / f64x8::splat(4.0)));
            let t222 = f64x8::splat(1.0) / t210;
            let t224 = (simd::erf(t222 / f64x8::splat(2.0)));
            let t227 = f64x8::splat(10.0) * t210 * t114 * t224 + t215 * t218 - f64x8::splat(36.0) * t211 + t214 - f64x8::splat(3.0);
            let t231 = ((t137).select(t175 + t208, f64x8::splat(24.0) * t211 * t227 + f64x8::splat(1.0)));
            let t232 = v_tau * t52;
            let t234 = f64x8::splat(1.0) / t55 / v_rho;
            let t235 = t232 * t234;
            let t236 = f64x8::splat(0.14554132) * t235;
            let t237 = t61 * t48;
            let t240 = -t236 + f64x8::splat(0.043662396) * t237 + f64x8::splat(0.04229627833333333) * t58;
            let t241 = t231 * t240;
            let t242 = t134 * t134;
            let t243 = f64x8::splat(1.0) / t242;
            let t244 = t50 * t243;
            let t247 = (f64x8::splat(0.32)).simd_le(t84);
            let t248 = (f64x8::splat(0.32)).simd_lt(t84);
            let t249 = ((t248).select(t84, f64x8::splat(0.32)));
            let t250 = t249 * t249;
            let t251 = t250 * t250;
            let t254 = t251 * t250;
            let t255 = f64x8::splat(1.0) / t254;
            let t257 = t251 * t251;
            let t258 = f64x8::splat(1.0) / t257;
            let t260 = t257 * t250;
            let t261 = f64x8::splat(1.0) / t260;
            let t263 = t257 * t251;
            let t264 = f64x8::splat(1.0) / t263;
            let t266 = t257 * t254;
            let t267 = f64x8::splat(1.0) / t266;
            let t269 = t257 * t257;
            let t270 = f64x8::splat(1.0) / t269;
            let t273 = f64x8::splat(1.0) / t269 / t250;
            let t276 = f64x8::splat(1.0) / t269 / t251;
            let t279 = f64x8::splat(1.0) / t269 / t254;
            let t282 = f64x8::splat(1.0) / t269 / t257;
            let t285 = f64x8::splat(1.0) / t269 / t260;
            let t288 = f64x8::splat(1.0) / t269 / t263;
            let t291 = f64x8::splat(1.0) / t269 / t266;
            let t293 = t269 * t269;
            let t294 = f64x8::splat(1.0) / t293;
            let t297 = f64x8::splat(1.0) / t293 / t250;
            let t300 = f64x8::splat(1.0) / t293 / t251;
            let t303 = f64x8::splat(1.0) / t293 / t254;
            let t305 = f64x8::splat(3.0) / f64x8::splat(7840.0) / t251 - t255 / f64x8::splat(56448.0) + f64x8::splat(5.0) / f64x8::splat(8515584.0) * t258 - t261 / f64x8::splat(61501440.0) + t264 / f64x8::splat(2530344960.0) - t267 / f64x8::splat(115811942400.0) + t270 / f64x8::splat(5811921223680.0) - t273 / f64x8::splat(316612955602944.0) + t276 / f64x8::splat(1.85827061661696e+16) - t279 / f64x8::splat(1.168055816159232e+18) + t282 / f64x8::splat(7.824446865801216e+19) - t285 / f64x8::splat(5.562511054710453e+21) + t288 / f64x8::splat(4.181740504354862e+23) - t291 / f64x8::splat(3.3139778504339334e+25) + t294 / f64x8::splat(2.7608516801793436e+27) - t297 / f64x8::splat(2.4119107039344544e+29) + t300 / f64x8::splat(2.2046293272414373e+31) - t303 / f64x8::splat(2.1042094544618633e+33);
            let t306 = ((t248).select(f64x8::splat(0.32), t84));
            let t308 = t306 * t306;
            let t309 = t308 * t306;
            let t311 = t308 * t308;
            let t312 = t311 * t306;
            let t314 = t311 * t309;
            let t316 = t311 * t311;
            let t317 = t316 * t306;
            let t319 = -f64x8::splat(8.0) * t306 + f64x8::splat(256.0) * t309 - f64x8::splat(576.0) * t312 + f64x8::splat(3840.0) * t314 - f64x8::splat(122880.0) * t317;
            let t320 = f64x8::splat(1.0) / t308;
            let t322 = (simd::exp(-t320 / f64x8::splat(4.0)));
            let t326 = t311 * t308;
            let t328 = -f64x8::splat(35.0) + f64x8::splat(224.0) * t308 - f64x8::splat(1440.0) * t311 + f64x8::splat(5120.0) * t326;
            let t332 = -f64x8::splat(2.0) + f64x8::splat(60.0) * t308;
            let t334 = f64x8::splat(1.0) / t306;
            let t336 = (simd::erf(t334 / f64x8::splat(2.0)));
            let t339 = f64x8::splat(2.0) * t114 * t332 * t336 + f64x8::splat(24.0) * t309 * t328 + t319 * t322;
            let t343 = ((t247).select(t305, f64x8::splat(1.0) + f64x8::splat(8.0) / f64x8::splat(7.0) * t306 * t339));
            let t344 = t343 * t45;
            let t345 = t344 * t49;
            let t347 = t53 * t57 * t243;
            let t350 = t133 * t135 + f64x8::splat(35.0) / f64x8::splat(81.0) * t241 * t244 + f64x8::splat(0.026329605555555555) * t345 * t347;
            let t352 = f64x8::splat(1.0) - t35;
            let t355 = t42 * t44 * t78 / f64x8::splat(18.0);
            let t356 = (f64x8::splat(1.35)).simd_le(t355);
            let t357 = (f64x8::splat(1.35)).simd_lt(t355);
            let t358 = ((t357).select(t355, f64x8::splat(1.35)));
            let t359 = t358 * t358;
            let t362 = t359 * t359;
            let t363 = f64x8::splat(1.0) / t362;
            let t365 = t362 * t359;
            let t366 = f64x8::splat(1.0) / t365;
            let t368 = t362 * t362;
            let t369 = f64x8::splat(1.0) / t368;
            let t372 = f64x8::splat(1.0) / t368 / t359;
            let t375 = f64x8::splat(1.0) / t368 / t362;
            let t378 = f64x8::splat(1.0) / t368 / t365;
            let t380 = t368 * t368;
            let t381 = f64x8::splat(1.0) / t380;
            let t384 = ((t357).select(f64x8::splat(1.35), t355));
            let t385 = f64x8::splat(1.0) / t384;
            let t387 = (simd::erf(t385 / f64x8::splat(2.0)));
            let t389 = t384 * t384;
            let t390 = f64x8::splat(1.0) / t389;
            let t392 = (simd::exp(-t390 / f64x8::splat(4.0)));
            let t393 = t392 - f64x8::splat(1.0);
            let t396 = t392 - f64x8::splat(3.0) / f64x8::splat(2.0) - f64x8::splat(2.0) * t389 * t393;
            let t399 = t114 * t387 + f64x8::splat(2.0) * t384 * t396;
            let t403 = ((t356).select(f64x8::splat(1.0) / t359 / f64x8::splat(36.0) - t363 / f64x8::splat(960.0) + t366 / f64x8::splat(26880.0) - t369 / f64x8::splat(829440.0) + t372 / f64x8::splat(28385280.0) - t375 / f64x8::splat(1073479680.0) + t378 / f64x8::splat(44590694400.0) - t381 / f64x8::splat(2021444812800.0), f64x8::splat(1.0) - f64x8::splat(8.0) / f64x8::splat(3.0) * t384 * t399));
            let t404 = t352 * t403;
            let t407 = (f64x8::splat(10.0) / f64x8::splat(81.0) + f64x8::splat(25.0) / f64x8::splat(8748.0) * t59) * t45;
            let t408 = t407 * t49;
            let t417 = (t235 - t58 / f64x8::splat(8.0)) * t45 * t49 / f64x8::splat(4.0) - f64x8::splat(9.0) / f64x8::splat(20.0) + t59 / f64x8::splat(36.0);
            let t418 = t417 * t417;
            let t420 = t417 * t27;
            let t421 = f64x8::splat(1.0) - t27;
            let t424 = f64x8::splat(1.0) + f64x8::splat(5.0) / f64x8::splat(12.0) * t408 * t58 + f64x8::splat(292.0) / f64x8::splat(405.0) * t418 - f64x8::splat(146.0) / f64x8::splat(135.0) * t420 * t421;
            let t425 = (simd::pow(t424, f64x8::splat(1.0) / f64x8::splat(10.0)));
            let t436 = f64x8::splat(1.0) + f64x8::splat(0.06394332777777778) * t59 - f64x8::splat(5.0) / f64x8::splat(9.0) * (t236 + f64x8::splat(0.256337604) * t237 + f64x8::splat(0.011867481666666667) * t58) * t45 * t49;
            let t439 = t135 + f64x8::splat(7.0) / f64x8::splat(9.0) * t436 * t243;
            let t442 = -param_hyb_coeff_0 * (t35 * t350 + t404 * t425) + t35 * t439 + t352 * t425;
            let t446 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t442));
            let tzk0 = f64x8::splat(2.0) * t446;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}
