//! HYB_MGGA_X_PJS18 exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/hyb_mgga_x_pjs18.c`
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

/// Store 8 elements with a given stride and offset.
#[inline(always)]
fn store_strided(s: &mut [f64], ip: usize, m: usize, stride: usize, offset: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let base = ip * stride + offset;
        s[base] = a[0];
        s[base + stride] = a[1];
        s[base + 2 * stride] = a[2];
        s[base + 3 * stride] = a[3];
        s[base + 4 * stride] = a[4];
        s[base + 5 * stride] = a[5];
        s[base + 6 * stride] = a[6];
        s[base + 7 * stride] = a[7];
    } else {
        for k in 0..m {
            s[(ip + k) * stride + offset] = a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn hyb_mgga_x_pjs18_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_hyb_omega_0 = f64x8::splat(param_hyb_omega_0);
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
        let v_lapl0 = load_strided(lapl, ip, np, 2, 0);
        let v_lapl1 = load_strided(lapl, ip, np, 2, 1);
        let v_tau0 = load_strided(tau, ip, np, 2, 0);
        let v_tau1 = load_strided(tau, ip, np, 2, 1);
        let mut acc_zk = V_ZERO;
        {
            let t2 = (v_rho0).simd_le(dens_threshold);
            let t3 = f64x8::splat(M_CBRT3);
            let t4 = f64x8::splat(M_CBRTPI);
            let t6 = t3 / t4;
            let t7 = v_rho0 + v_rho1;
            let t8 = f64x8::splat(1.0) / t7;
            let t11 = (f64x8::splat(2.0) * v_rho0 * t8).simd_le(zeta_threshold);
            let t12 = zeta_threshold - f64x8::splat(1.0);
            let t15 = (f64x8::splat(2.0) * v_rho1 * t8).simd_le(zeta_threshold);
            let t16 = -t12;
            let t17 = v_rho0 - v_rho1;
            let t18 = t17 * t8;
            let t19 = ((t11).select(t12, (t15).select(t16, t18)));
            let t20 = f64x8::splat(1.0) + t19;
            let t21 = (t20).simd_le(zeta_threshold);
            let t22 = (simd::cbrt(zeta_threshold));
            let t23 = t22 * zeta_threshold;
            let t24 = (simd::cbrt(t20));
            let t26 = ((t21).select(t23, t24 * t20));
            let t27 = (simd::cbrt(t7));
            let t28 = t26 * t27;
            let t29 = (simd::cbrt(f64x8::splat(9.0)));
            let t30 = t29 * t29;
            let t32 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t33 = t32 * t32;
            let t34 = t30 * t33;
            let t35 = t34 * param_hyb_omega_0;
            let t36 = f64x8::splat(1.0) / t27;
            let t37 = t3 * t36;
            let t38 = f64x8::splat(M_CBRT6);
            let t39 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t40 = (simd::cbrt(t39));
            let t41 = t40 * t40;
            let t42 = f64x8::splat(1.0) / t41;
            let t43 = t38 * t42;
            let t44 = v_rho0 * v_rho0;
            let t45 = (simd::cbrt(v_rho0));
            let t46 = t45 * t45;
            let t48 = f64x8::splat(1.0) / t46 / t44;
            let t49 = v_sigma0 * t48;
            let t52 = t38 * t38;
            let t54 = f64x8::splat(1.0) / t40 / t39;
            let t55 = t52 * t54;
            let t56 = v_sigma0 * v_sigma0;
            let t57 = t44 * t44;
            let t58 = t57 * v_rho0;
            let t60 = f64x8::splat(1.0) / t45 / t58;
            let t64 = f64x8::splat(1.0) + f64x8::splat(0.1504548888888889) * t43 * t49 + f64x8::splat(0.002689949046226295) * t55 * t56 * t60;
            let t65 = (simd::pow(t64, f64x8::splat(1.0) / f64x8::splat(10.0)));
            let t66 = f64x8::splat(1.0) / t65;
            let t68 = (f64x8::splat(1.0) + t18).simd_le(zeta_threshold);
            let t70 = (f64x8::splat(1.0) - t18).simd_le(zeta_threshold);
            let t71 = ((t68).select(t12, (t70).select(t16, t18)));
            let t72 = f64x8::splat(1.0) + t71;
            let t73 = (t72).simd_le(zeta_threshold);
            let t74 = (simd::cbrt(t72));
            let t75 = ((t73).select(t22, t74));
            let t76 = f64x8::splat(1.0) / t75;
            let t77 = t66 * t76;
            let t80 = t35 * t37 * t77 / f64x8::splat(18.0);
            let t81 = (t80).simd_lt(f64x8::splat(1e-10));
            let t82 = ((t81).select(f64x8::splat(1e-10), t80));
            let t83 = (f64x8::splat(1.35)).simd_le(t82);
            let t84 = (f64x8::splat(1.35)).simd_lt(t82);
            let t85 = ((t84).select(t82, f64x8::splat(1.35)));
            let t86 = t85 * t85;
            let t89 = t86 * t86;
            let t90 = f64x8::splat(1.0) / t89;
            let t92 = t89 * t86;
            let t93 = f64x8::splat(1.0) / t92;
            let t95 = t89 * t89;
            let t96 = f64x8::splat(1.0) / t95;
            let t99 = f64x8::splat(1.0) / t95 / t86;
            let t102 = f64x8::splat(1.0) / t95 / t89;
            let t105 = f64x8::splat(1.0) / t95 / t92;
            let t107 = t95 * t95;
            let t108 = f64x8::splat(1.0) / t107;
            let t111 = ((t84).select(f64x8::splat(1.35), t82));
            let t112 = ((f64x8::splat(M_PI)).sqrt());
            let t113 = f64x8::splat(1.0) / t111;
            let t115 = (simd::erf(t113 / f64x8::splat(2.0)));
            let t117 = t111 * t111;
            let t118 = f64x8::splat(1.0) / t117;
            let t120 = (simd::exp(-t118 / f64x8::splat(4.0)));
            let t121 = t120 - f64x8::splat(1.0);
            let t124 = t120 - f64x8::splat(3.0) / f64x8::splat(2.0) - f64x8::splat(2.0) * t117 * t121;
            let t127 = f64x8::splat(2.0) * t111 * t124 + t112 * t115;
            let t131 = ((t83).select(f64x8::splat(1.0) / t86 / f64x8::splat(36.0) - t90 / f64x8::splat(960.0) + t93 / f64x8::splat(26880.0) - t96 / f64x8::splat(829440.0) + t99 / f64x8::splat(28385280.0) - t102 / f64x8::splat(1073479680.0) + t105 / f64x8::splat(44590694400.0) - t108 / f64x8::splat(2021444812800.0), f64x8::splat(1.0) - f64x8::splat(8.0) / f64x8::splat(3.0) * t111 * t127));
            let t132 = (simd::pow(t64, f64x8::splat(1.0) / f64x8::splat(5.0)));
            let t133 = f64x8::splat(1.0) / t132;
            let t135 = (f64x8::splat(0.27)).simd_le(t82);
            let t136 = (f64x8::splat(0.27)).simd_lt(t82);
            let t137 = ((t136).select(t82, f64x8::splat(0.27)));
            let t138 = t137 * t137;
            let t139 = t138 * t138;
            let t140 = t139 * t139;
            let t141 = t140 * t139;
            let t142 = t140 * t140;
            let t143 = t142 * t142;
            let t145 = f64x8::splat(1.0) / t143 / t141;
            let t147 = t139 * t138;
            let t148 = t140 * t147;
            let t150 = f64x8::splat(1.0) / t143 / t148;
            let t154 = f64x8::splat(1.0) / t147;
            let t156 = f64x8::splat(1.0) / t140;
            let t158 = t140 * t138;
            let t159 = f64x8::splat(1.0) / t158;
            let t161 = f64x8::splat(1.0) / t141;
            let t163 = f64x8::splat(1.0) / t148;
            let t165 = f64x8::splat(1.0) / t142;
            let t167 = t142 * t138;
            let t168 = f64x8::splat(1.0) / t167;
            let t171 = f64x8::splat(1.0) / t142 / t139;
            let t173 = t145 / f64x8::splat(3.3929038000650147e+37) - t150 / f64x8::splat(3.511556992918352e+39) + f64x8::splat(3.0) / f64x8::splat(2240.0) / t139 - t154 / f64x8::splat(11520.0) + f64x8::splat(3.0) / f64x8::splat(788480.0) * t156 - t159 / f64x8::splat(7454720.0) + t161 / f64x8::splat(247726080.0) - t163 / f64x8::splat(9358540800.0) + t165 / f64x8::splat(394474291200.0) - t168 / f64x8::splat(18311911833600.0) + t171 / f64x8::splat(927028425523200.0);
            let t175 = f64x8::splat(1.0) / t142 / t147;
            let t178 = f64x8::splat(1.0) / t142 / t140;
            let t181 = f64x8::splat(1.0) / t142 / t158;
            let t184 = f64x8::splat(1.0) / t142 / t141;
            let t187 = f64x8::splat(1.0) / t142 / t148;
            let t189 = f64x8::splat(1.0) / t143;
            let t192 = f64x8::splat(1.0) / t143 / t138;
            let t195 = f64x8::splat(1.0) / t143 / t139;
            let t198 = f64x8::splat(1.0) / t143 / t147;
            let t201 = f64x8::splat(1.0) / t143 / t140;
            let t204 = f64x8::splat(1.0) / t143 / t158;
            let t206 = -t175 / f64x8::splat(5.0785035485184e+16) + t178 / f64x8::splat(2.991700272218112e+18) - t181 / f64x8::splat(1.88514051721003e+20) + t184 / f64x8::splat(1.2648942844388573e+22) - t187 / f64x8::splat(9.002316741416457e+23) + t189 / f64x8::splat(6.772652029299977e+25) - t192 / f64x8::splat(5.36974553751641e+27) + t195 / f64x8::splat(4.474731034888079e+29) - t198 / f64x8::splat(3.909716563474291e+31) + t201 / f64x8::splat(3.5738523369945735e+33) - t204 / f64x8::splat(3.410951160703658e+35);
            let t208 = ((t136).select(f64x8::splat(0.27), t82));
            let t209 = t208 * t208;
            let t211 = t209 * t209;
            let t212 = f64x8::splat(64.0) * t211;
            let t213 = f64x8::splat(20.0) * t209 - t212;
            let t216 = (simd::exp(-f64x8::splat(1.0) / t209 / f64x8::splat(4.0)));
            let t220 = f64x8::splat(1.0) / t208;
            let t222 = (simd::erf(t220 / f64x8::splat(2.0)));
            let t225 = f64x8::splat(10.0) * t208 * t112 * t222 + t213 * t216 - f64x8::splat(36.0) * t209 + t212 - f64x8::splat(3.0);
            let t229 = ((t135).select(t173 + t206, f64x8::splat(24.0) * t209 * t225 + f64x8::splat(1.0)));
            let t231 = f64x8::splat(1.0) / t46 / v_rho0;
            let t235 = f64x8::splat(0.043662396) * t52 * t41;
            let t237 = -f64x8::splat(0.14554132) * v_tau0 * t231 + t235 + f64x8::splat(0.04229627833333333) * t49;
            let t238 = t229 * t237;
            let t239 = t132 * t132;
            let t240 = f64x8::splat(1.0) / t239;
            let t241 = t43 * t240;
            let t244 = (f64x8::splat(0.32)).simd_le(t82);
            let t245 = (f64x8::splat(0.32)).simd_lt(t82);
            let t246 = ((t245).select(t82, f64x8::splat(0.32)));
            let t247 = t246 * t246;
            let t248 = t247 * t247;
            let t251 = t248 * t247;
            let t252 = f64x8::splat(1.0) / t251;
            let t254 = t248 * t248;
            let t255 = f64x8::splat(1.0) / t254;
            let t257 = t254 * t247;
            let t258 = f64x8::splat(1.0) / t257;
            let t260 = t254 * t248;
            let t261 = f64x8::splat(1.0) / t260;
            let t263 = t254 * t251;
            let t264 = f64x8::splat(1.0) / t263;
            let t266 = t254 * t254;
            let t267 = f64x8::splat(1.0) / t266;
            let t270 = f64x8::splat(1.0) / t266 / t247;
            let t273 = f64x8::splat(1.0) / t266 / t248;
            let t276 = f64x8::splat(1.0) / t266 / t251;
            let t279 = f64x8::splat(1.0) / t266 / t254;
            let t282 = f64x8::splat(1.0) / t266 / t257;
            let t285 = f64x8::splat(1.0) / t266 / t260;
            let t288 = f64x8::splat(1.0) / t266 / t263;
            let t290 = t266 * t266;
            let t291 = f64x8::splat(1.0) / t290;
            let t294 = f64x8::splat(1.0) / t290 / t247;
            let t297 = f64x8::splat(1.0) / t290 / t248;
            let t300 = f64x8::splat(1.0) / t290 / t251;
            let t302 = f64x8::splat(3.0) / f64x8::splat(7840.0) / t248 - t252 / f64x8::splat(56448.0) + f64x8::splat(5.0) / f64x8::splat(8515584.0) * t255 - t258 / f64x8::splat(61501440.0) + t261 / f64x8::splat(2530344960.0) - t264 / f64x8::splat(115811942400.0) + t267 / f64x8::splat(5811921223680.0) - t270 / f64x8::splat(316612955602944.0) + t273 / f64x8::splat(1.85827061661696e+16) - t276 / f64x8::splat(1.168055816159232e+18) + t279 / f64x8::splat(7.824446865801216e+19) - t282 / f64x8::splat(5.562511054710453e+21) + t285 / f64x8::splat(4.181740504354862e+23) - t288 / f64x8::splat(3.3139778504339334e+25) + t291 / f64x8::splat(2.7608516801793436e+27) - t294 / f64x8::splat(2.4119107039344544e+29) + t297 / f64x8::splat(2.2046293272414373e+31) - t300 / f64x8::splat(2.1042094544618633e+33);
            let t303 = ((t245).select(f64x8::splat(0.32), t82));
            let t305 = t303 * t303;
            let t306 = t305 * t303;
            let t308 = t305 * t305;
            let t309 = t308 * t303;
            let t311 = t308 * t306;
            let t313 = t308 * t308;
            let t314 = t313 * t303;
            let t316 = -f64x8::splat(8.0) * t303 + f64x8::splat(256.0) * t306 - f64x8::splat(576.0) * t309 + f64x8::splat(3840.0) * t311 - f64x8::splat(122880.0) * t314;
            let t317 = f64x8::splat(1.0) / t305;
            let t319 = (simd::exp(-t317 / f64x8::splat(4.0)));
            let t323 = t308 * t305;
            let t325 = -f64x8::splat(35.0) + f64x8::splat(224.0) * t305 - f64x8::splat(1440.0) * t308 + f64x8::splat(5120.0) * t323;
            let t329 = -f64x8::splat(2.0) + f64x8::splat(60.0) * t305;
            let t331 = f64x8::splat(1.0) / t303;
            let t333 = (simd::erf(t331 / f64x8::splat(2.0)));
            let t336 = f64x8::splat(2.0) * t112 * t329 * t333 + f64x8::splat(24.0) * t306 * t325 + t316 * t319;
            let t340 = ((t244).select(t302, f64x8::splat(1.0) + f64x8::splat(8.0) / f64x8::splat(7.0) * t303 * t336));
            let t341 = t340 * t38;
            let t342 = t341 * t42;
            let t343 = t49 * t240;
            let t346 = t131 * t133 + f64x8::splat(35.0) / f64x8::splat(81.0) * t238 * t241 + f64x8::splat(0.026329605555555555) * t342 * t343;
            let t350 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t346));
            let t351 = (v_rho1).simd_le(dens_threshold);
            let t352 = -t17;
            let t354 = ((t15).select(t12, (t11).select(t16, t352 * t8)));
            let t355 = f64x8::splat(1.0) + t354;
            let t356 = (t355).simd_le(zeta_threshold);
            let t357 = (simd::cbrt(t355));
            let t359 = ((t356).select(t23, t357 * t355));
            let t360 = t359 * t27;
            let t361 = v_rho1 * v_rho1;
            let t362 = (simd::cbrt(v_rho1));
            let t363 = t362 * t362;
            let t365 = f64x8::splat(1.0) / t363 / t361;
            let t366 = v_sigma2 * t365;
            let t369 = v_sigma2 * v_sigma2;
            let t370 = t361 * t361;
            let t371 = t370 * v_rho1;
            let t373 = f64x8::splat(1.0) / t362 / t371;
            let t377 = f64x8::splat(1.0) + f64x8::splat(0.1504548888888889) * t43 * t366 + f64x8::splat(0.002689949046226295) * t55 * t369 * t373;
            let t378 = (simd::pow(t377, f64x8::splat(1.0) / f64x8::splat(10.0)));
            let t379 = f64x8::splat(1.0) / t378;
            let t380 = ((t70).select(t12, (t68).select(t16, -t18)));
            let t381 = f64x8::splat(1.0) + t380;
            let t382 = (t381).simd_le(zeta_threshold);
            let t383 = (simd::cbrt(t381));
            let t384 = ((t382).select(t22, t383));
            let t385 = f64x8::splat(1.0) / t384;
            let t386 = t379 * t385;
            let t389 = t35 * t37 * t386 / f64x8::splat(18.0);
            let t390 = (t389).simd_lt(f64x8::splat(1e-10));
            let t391 = ((t390).select(f64x8::splat(1e-10), t389));
            let t392 = (f64x8::splat(1.35)).simd_le(t391);
            let t393 = (f64x8::splat(1.35)).simd_lt(t391);
            let t394 = ((t393).select(t391, f64x8::splat(1.35)));
            let t395 = t394 * t394;
            let t398 = t395 * t395;
            let t399 = f64x8::splat(1.0) / t398;
            let t401 = t398 * t395;
            let t402 = f64x8::splat(1.0) / t401;
            let t404 = t398 * t398;
            let t405 = f64x8::splat(1.0) / t404;
            let t408 = f64x8::splat(1.0) / t404 / t395;
            let t411 = f64x8::splat(1.0) / t404 / t398;
            let t414 = f64x8::splat(1.0) / t404 / t401;
            let t416 = t404 * t404;
            let t417 = f64x8::splat(1.0) / t416;
            let t420 = ((t393).select(f64x8::splat(1.35), t391));
            let t421 = f64x8::splat(1.0) / t420;
            let t423 = (simd::erf(t421 / f64x8::splat(2.0)));
            let t425 = t420 * t420;
            let t426 = f64x8::splat(1.0) / t425;
            let t428 = (simd::exp(-t426 / f64x8::splat(4.0)));
            let t429 = t428 - f64x8::splat(1.0);
            let t432 = t428 - f64x8::splat(3.0) / f64x8::splat(2.0) - f64x8::splat(2.0) * t425 * t429;
            let t435 = t112 * t423 + f64x8::splat(2.0) * t420 * t432;
            let t439 = ((t392).select(f64x8::splat(1.0) / t395 / f64x8::splat(36.0) - t399 / f64x8::splat(960.0) + t402 / f64x8::splat(26880.0) - t405 / f64x8::splat(829440.0) + t408 / f64x8::splat(28385280.0) - t411 / f64x8::splat(1073479680.0) + t414 / f64x8::splat(44590694400.0) - t417 / f64x8::splat(2021444812800.0), f64x8::splat(1.0) - f64x8::splat(8.0) / f64x8::splat(3.0) * t420 * t435));
            let t440 = (simd::pow(t377, f64x8::splat(1.0) / f64x8::splat(5.0)));
            let t441 = f64x8::splat(1.0) / t440;
            let t443 = (f64x8::splat(0.27)).simd_le(t391);
            let t444 = (f64x8::splat(0.27)).simd_lt(t391);
            let t445 = ((t444).select(t391, f64x8::splat(0.27)));
            let t446 = t445 * t445;
            let t447 = t446 * t446;
            let t450 = t447 * t446;
            let t451 = f64x8::splat(1.0) / t450;
            let t453 = t447 * t447;
            let t454 = f64x8::splat(1.0) / t453;
            let t456 = t453 * t446;
            let t457 = f64x8::splat(1.0) / t456;
            let t459 = t453 * t447;
            let t460 = f64x8::splat(1.0) / t459;
            let t462 = t453 * t450;
            let t463 = f64x8::splat(1.0) / t462;
            let t465 = t453 * t453;
            let t466 = f64x8::splat(1.0) / t465;
            let t468 = t465 * t446;
            let t469 = f64x8::splat(1.0) / t468;
            let t472 = f64x8::splat(1.0) / t465 / t447;
            let t475 = f64x8::splat(1.0) / t465 / t450;
            let t478 = f64x8::splat(1.0) / t465 / t453;
            let t480 = f64x8::splat(3.0) / f64x8::splat(2240.0) / t447 - t451 / f64x8::splat(11520.0) + f64x8::splat(3.0) / f64x8::splat(788480.0) * t454 - t457 / f64x8::splat(7454720.0) + t460 / f64x8::splat(247726080.0) - t463 / f64x8::splat(9358540800.0) + t466 / f64x8::splat(394474291200.0) - t469 / f64x8::splat(18311911833600.0) + t472 / f64x8::splat(927028425523200.0) - t475 / f64x8::splat(5.0785035485184e+16) + t478 / f64x8::splat(2.991700272218112e+18);
            let t482 = f64x8::splat(1.0) / t465 / t456;
            let t485 = f64x8::splat(1.0) / t465 / t459;
            let t488 = f64x8::splat(1.0) / t465 / t462;
            let t490 = t465 * t465;
            let t491 = f64x8::splat(1.0) / t490;
            let t494 = f64x8::splat(1.0) / t490 / t446;
            let t497 = f64x8::splat(1.0) / t490 / t447;
            let t500 = f64x8::splat(1.0) / t490 / t450;
            let t503 = f64x8::splat(1.0) / t490 / t453;
            let t506 = f64x8::splat(1.0) / t490 / t456;
            let t509 = f64x8::splat(1.0) / t490 / t459;
            let t512 = f64x8::splat(1.0) / t490 / t462;
            let t514 = -t482 / f64x8::splat(1.88514051721003e+20) + t485 / f64x8::splat(1.2648942844388573e+22) - t488 / f64x8::splat(9.002316741416457e+23) + t491 / f64x8::splat(6.772652029299977e+25) - t494 / f64x8::splat(5.36974553751641e+27) + t497 / f64x8::splat(4.474731034888079e+29) - t500 / f64x8::splat(3.909716563474291e+31) + t503 / f64x8::splat(3.5738523369945735e+33) - t506 / f64x8::splat(3.410951160703658e+35) + t509 / f64x8::splat(3.3929038000650147e+37) - t512 / f64x8::splat(3.511556992918352e+39);
            let t516 = ((t444).select(f64x8::splat(0.27), t391));
            let t517 = t516 * t516;
            let t519 = t517 * t517;
            let t520 = f64x8::splat(64.0) * t519;
            let t521 = f64x8::splat(20.0) * t517 - t520;
            let t524 = (simd::exp(-f64x8::splat(1.0) / t517 / f64x8::splat(4.0)));
            let t528 = f64x8::splat(1.0) / t516;
            let t530 = (simd::erf(t528 / f64x8::splat(2.0)));
            let t533 = f64x8::splat(10.0) * t516 * t112 * t530 + t521 * t524 - f64x8::splat(36.0) * t517 + t520 - f64x8::splat(3.0);
            let t537 = ((t443).select(t480 + t514, f64x8::splat(24.0) * t517 * t533 + f64x8::splat(1.0)));
            let t539 = f64x8::splat(1.0) / t363 / v_rho1;
            let t543 = -f64x8::splat(0.14554132) * v_tau1 * t539 + t235 + f64x8::splat(0.04229627833333333) * t366;
            let t544 = t537 * t543;
            let t545 = t440 * t440;
            let t546 = f64x8::splat(1.0) / t545;
            let t547 = t43 * t546;
            let t550 = (f64x8::splat(0.32)).simd_le(t391);
            let t551 = (f64x8::splat(0.32)).simd_lt(t391);
            let t552 = ((t551).select(t391, f64x8::splat(0.32)));
            let t553 = t552 * t552;
            let t554 = t553 * t553;
            let t557 = t554 * t553;
            let t558 = f64x8::splat(1.0) / t557;
            let t560 = t554 * t554;
            let t561 = f64x8::splat(1.0) / t560;
            let t563 = t560 * t553;
            let t564 = f64x8::splat(1.0) / t563;
            let t566 = t560 * t554;
            let t567 = f64x8::splat(1.0) / t566;
            let t569 = t560 * t557;
            let t570 = f64x8::splat(1.0) / t569;
            let t572 = t560 * t560;
            let t573 = f64x8::splat(1.0) / t572;
            let t576 = f64x8::splat(1.0) / t572 / t553;
            let t579 = f64x8::splat(1.0) / t572 / t554;
            let t582 = f64x8::splat(1.0) / t572 / t557;
            let t585 = f64x8::splat(1.0) / t572 / t560;
            let t588 = f64x8::splat(1.0) / t572 / t563;
            let t591 = f64x8::splat(1.0) / t572 / t566;
            let t594 = f64x8::splat(1.0) / t572 / t569;
            let t596 = t572 * t572;
            let t597 = f64x8::splat(1.0) / t596;
            let t600 = f64x8::splat(1.0) / t596 / t553;
            let t603 = f64x8::splat(1.0) / t596 / t554;
            let t606 = f64x8::splat(1.0) / t596 / t557;
            let t608 = f64x8::splat(3.0) / f64x8::splat(7840.0) / t554 - t558 / f64x8::splat(56448.0) + f64x8::splat(5.0) / f64x8::splat(8515584.0) * t561 - t564 / f64x8::splat(61501440.0) + t567 / f64x8::splat(2530344960.0) - t570 / f64x8::splat(115811942400.0) + t573 / f64x8::splat(5811921223680.0) - t576 / f64x8::splat(316612955602944.0) + t579 / f64x8::splat(1.85827061661696e+16) - t582 / f64x8::splat(1.168055816159232e+18) + t585 / f64x8::splat(7.824446865801216e+19) - t588 / f64x8::splat(5.562511054710453e+21) + t591 / f64x8::splat(4.181740504354862e+23) - t594 / f64x8::splat(3.3139778504339334e+25) + t597 / f64x8::splat(2.7608516801793436e+27) - t600 / f64x8::splat(2.4119107039344544e+29) + t603 / f64x8::splat(2.2046293272414373e+31) - t606 / f64x8::splat(2.1042094544618633e+33);
            let t609 = ((t551).select(f64x8::splat(0.32), t391));
            let t611 = t609 * t609;
            let t612 = t611 * t609;
            let t614 = t611 * t611;
            let t615 = t614 * t609;
            let t617 = t614 * t612;
            let t619 = t614 * t614;
            let t620 = t619 * t609;
            let t622 = -f64x8::splat(8.0) * t609 + f64x8::splat(256.0) * t612 - f64x8::splat(576.0) * t615 + f64x8::splat(3840.0) * t617 - f64x8::splat(122880.0) * t620;
            let t623 = f64x8::splat(1.0) / t611;
            let t625 = (simd::exp(-t623 / f64x8::splat(4.0)));
            let t629 = t614 * t611;
            let t631 = -f64x8::splat(35.0) + f64x8::splat(224.0) * t611 - f64x8::splat(1440.0) * t614 + f64x8::splat(5120.0) * t629;
            let t635 = -f64x8::splat(2.0) + f64x8::splat(60.0) * t611;
            let t637 = f64x8::splat(1.0) / t609;
            let t639 = (simd::erf(t637 / f64x8::splat(2.0)));
            let t642 = f64x8::splat(2.0) * t112 * t635 * t639 + f64x8::splat(24.0) * t612 * t631 + t622 * t625;
            let t646 = ((t550).select(t608, f64x8::splat(1.0) + f64x8::splat(8.0) / f64x8::splat(7.0) * t609 * t642));
            let t647 = t646 * t38;
            let t648 = t647 * t42;
            let t649 = t366 * t546;
            let t652 = t439 * t441 + f64x8::splat(35.0) / f64x8::splat(81.0) * t544 * t547 + f64x8::splat(0.026329605555555555) * t648 * t649;
            let t656 = ((t351).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t360 * t652));
            let tzk0 = t350 + t656;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
