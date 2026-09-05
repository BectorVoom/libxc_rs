//! LDA_C_PW kxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_pw.c`
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
pub fn lda_c_pw_kxc_pol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    v3rho3: &mut [f64],
    param_a_0: f64,
    param_alpha1_0: f64,
    param_beta1_0: f64,
    param_beta2_0: f64,
    param_beta3_0: f64,
    param_pp_0: f64,
    param_beta4_0: f64,
    param_a_2: f64,
    param_alpha1_2: f64,
    param_beta1_2: f64,
    param_beta2_2: f64,
    param_beta3_2: f64,
    param_pp_2: f64,
    param_beta4_2: f64,
    param_fz20: f64,
    param_a_1: f64,
    param_alpha1_1: f64,
    param_beta1_1: f64,
    param_beta2_1: f64,
    param_beta3_1: f64,
    param_pp_1: f64,
    param_beta4_1: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_a_0 = f64x8::splat(param_a_0);
    let param_alpha1_0 = f64x8::splat(param_alpha1_0);
    let param_beta1_0 = f64x8::splat(param_beta1_0);
    let param_beta2_0 = f64x8::splat(param_beta2_0);
    let param_beta3_0 = f64x8::splat(param_beta3_0);
    let param_pp_0 = f64x8::splat(param_pp_0);
    let param_beta4_0 = f64x8::splat(param_beta4_0);
    let param_a_2 = f64x8::splat(param_a_2);
    let param_alpha1_2 = f64x8::splat(param_alpha1_2);
    let param_beta1_2 = f64x8::splat(param_beta1_2);
    let param_beta2_2 = f64x8::splat(param_beta2_2);
    let param_beta3_2 = f64x8::splat(param_beta3_2);
    let param_pp_2 = f64x8::splat(param_pp_2);
    let param_beta4_2 = f64x8::splat(param_beta4_2);
    let param_fz20 = f64x8::splat(param_fz20);
    let param_a_1 = f64x8::splat(param_a_1);
    let param_alpha1_1 = f64x8::splat(param_alpha1_1);
    let param_beta1_1 = f64x8::splat(param_beta1_1);
    let param_beta2_1 = f64x8::splat(param_beta2_1);
    let param_beta3_1 = f64x8::splat(param_beta3_1);
    let param_pp_1 = f64x8::splat(param_pp_1);
    let param_beta4_1 = f64x8::splat(param_beta4_1);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho0 = load_strided(rho, ip, np, 2, 0);
        let v_rho1 = load_strided(rho, ip, np, 2, 1);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        let mut acc_v2rho2_0 = V_ZERO;
        let mut acc_v2rho2_1 = V_ZERO;
        let mut acc_v2rho2_2 = V_ZERO;
        let mut acc_v3rho3_0 = V_ZERO;
        let mut acc_v3rho3_1 = V_ZERO;
        let mut acc_v3rho3_2 = V_ZERO;
        let mut acc_v3rho3_3 = V_ZERO;
        {
            let t1 = param_a_0;
            let t2 = param_alpha1_0;
            let t3 = f64x8::splat(M_CBRT3);
            let t4 = t2 * t3;
            let t5 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t6 = (simd::cbrt(t5));
            let t7 = f64x8::splat(M_CBRT4);
            let t8 = t7 * t7;
            let t9 = t6 * t8;
            let t10 = v_rho0 + v_rho1;
            let t11 = (simd::cbrt(t10));
            let t12 = f64x8::splat(1.0) / t11;
            let t13 = t9 * t12;
            let t16 = f64x8::splat(1.0) + t4 * t13 / f64x8::splat(4.0);
            let t18 = f64x8::splat(1.0) / t1;
            let t19 = param_beta1_0;
            let t20 = t3 * t6;
            let t22 = t20 * t8 * t12;
            let t23 = ((t22).sqrt());
            let t27 = param_beta2_0 * t3;
            let t30 = param_beta3_0;
            let t31 = ((t22) * (t22).sqrt());
            let t35 = t22 / f64x8::splat(4.0);
            let t37 = param_pp_0 + f64x8::splat(1.0);
            let t38 = (simd::pow(t35, t37));
            let t39 = param_beta4_0 * t38;
            let t40 = t19 * t23 / f64x8::splat(2.0) + t27 * t13 / f64x8::splat(4.0) + f64x8::splat(0.125) * t30 * t31 + t39;
            let t44 = f64x8::splat(1.0) + t18 / t40 / f64x8::splat(2.0);
            let t45 = (simd::ln(t44));
            let t46 = t1 * t16 * t45;
            let t47 = f64x8::splat(2.0) * t46;
            let t48 = v_rho0 - v_rho1;
            let t49 = t48 * t48;
            let t50 = t49 * t49;
            let t51 = t10 * t10;
            let t52 = t51 * t51;
            let t53 = f64x8::splat(1.0) / t52;
            let t54 = t50 * t53;
            let t55 = f64x8::splat(1.0) / t10;
            let t56 = t48 * t55;
            let t57 = f64x8::splat(1.0) + t56;
            let t58 = (t57).simd_le(zeta_threshold);
            let t59 = (simd::cbrt(zeta_threshold));
            let t60 = t59 * zeta_threshold;
            let t61 = (simd::cbrt(t57));
            let t63 = ((t58).select(t60, t61 * t57));
            let t64 = f64x8::splat(1.0) - t56;
            let t65 = (t64).simd_le(zeta_threshold);
            let t66 = (simd::cbrt(t64));
            let t68 = ((t65).select(t60, t66 * t64));
            let t69 = t63 + t68 - f64x8::splat(2.0);
            let t70 = f64x8::splat(M_CBRT2);
            let t73 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t70 - f64x8::splat(2.0));
            let t74 = t69 * t73;
            let t75 = param_a_1;
            let t76 = param_alpha1_1;
            let t77 = t76 * t3;
            let t80 = f64x8::splat(1.0) + t77 * t13 / f64x8::splat(4.0);
            let t82 = f64x8::splat(1.0) / t75;
            let t83 = param_beta1_1;
            let t87 = param_beta2_1 * t3;
            let t90 = param_beta3_1;
            let t95 = param_pp_1 + f64x8::splat(1.0);
            let t96 = (simd::pow(t35, t95));
            let t97 = param_beta4_1 * t96;
            let t98 = t83 * t23 / f64x8::splat(2.0) + t87 * t13 / f64x8::splat(4.0) + f64x8::splat(0.125) * t90 * t31 + t97;
            let t102 = f64x8::splat(1.0) + t82 / t98 / f64x8::splat(2.0);
            let t103 = (simd::ln(t102));
            let t105 = param_a_2;
            let t106 = param_alpha1_2;
            let t107 = t106 * t3;
            let t110 = f64x8::splat(1.0) + t107 * t13 / f64x8::splat(4.0);
            let t112 = f64x8::splat(1.0) / t105;
            let t113 = param_beta1_2;
            let t117 = param_beta2_2 * t3;
            let t120 = param_beta3_2;
            let t125 = param_pp_2 + f64x8::splat(1.0);
            let t126 = (simd::pow(t35, t125));
            let t127 = param_beta4_2 * t126;
            let t128 = t113 * t23 / f64x8::splat(2.0) + t117 * t13 / f64x8::splat(4.0) + f64x8::splat(0.125) * t120 * t31 + t127;
            let t132 = f64x8::splat(1.0) + t112 / t128 / f64x8::splat(2.0);
            let t133 = (simd::ln(t132));
            let t134 = f64x8::splat(1.0) / param_fz20;
            let t135 = t133 * t134;
            let t138 = -f64x8::splat(2.0) * t75 * t80 * t103 - f64x8::splat(2.0) * t105 * t110 * t135 + f64x8::splat(2.0) * t46;
            let t139 = t74 * t138;
            let t140 = t54 * t139;
            let t143 = t110 * t133 * t134;
            let t145 = f64x8::splat(2.0) * t74 * t105 * t143;
            let tzk0 = -t47 + t140 + t145;
            acc_zk = tzk0;
            let t147 = t1 * t2 * t3;
            let t149 = f64x8::splat(1.0) / t11 / t10;
            let t152 = t147 * t9 * t149 * t45;
            let t153 = t152 / f64x8::splat(6.0);
            let t154 = t40 * t40;
            let t155 = f64x8::splat(1.0) / t154;
            let t156 = t16 * t155;
            let t157 = f64x8::splat(1.0) / t23;
            let t159 = t19 * t157 * t3;
            let t160 = t9 * t149;
            let t165 = ((t22).sqrt());
            let t167 = t30 * t165 * t3;
            let t173 = -t159 * t160 / f64x8::splat(12.0) - t27 * t160 / f64x8::splat(12.0) - f64x8::splat(0.0625) * t167 * t160 - t39 * t37 * t55 / f64x8::splat(3.0);
            let t174 = f64x8::splat(1.0) / t44;
            let t175 = t173 * t174;
            let t176 = t156 * t175;
            let t177 = t49 * t48;
            let t178 = t177 * t53;
            let t179 = t178 * t139;
            let t180 = f64x8::splat(4.0) * t179;
            let t181 = t52 * t10;
            let t182 = f64x8::splat(1.0) / t181;
            let t183 = t50 * t182;
            let t184 = t183 * t139;
            let t185 = f64x8::splat(4.0) * t184;
            let t186 = f64x8::splat(1.0) / t51;
            let t187 = t48 * t186;
            let t188 = t55 - t187;
            let t191 = ((t58).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t61 * t188));
            let t192 = -t188;
            let t195 = ((t65).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t66 * t192));
            let t197 = (t191 + t195) * t73;
            let t198 = t197 * t138;
            let t199 = t54 * t198;
            let t201 = t75 * t76 * t3;
            let t206 = t98 * t98;
            let t207 = f64x8::splat(1.0) / t206;
            let t208 = t80 * t207;
            let t210 = t83 * t157 * t3;
            let t216 = t90 * t165 * t3;
            let t222 = -t210 * t160 / f64x8::splat(12.0) - t87 * t160 / f64x8::splat(12.0) - f64x8::splat(0.0625) * t216 * t160 - t97 * t95 * t55 / f64x8::splat(3.0);
            let t223 = f64x8::splat(1.0) / t102;
            let t224 = t222 * t223;
            let t226 = t105 * t106;
            let t227 = t226 * t20;
            let t228 = t8 * t149;
            let t232 = t128 * t128;
            let t233 = f64x8::splat(1.0) / t232;
            let t234 = t110 * t233;
            let t236 = t113 * t157 * t3;
            let t242 = t120 * t165 * t3;
            let t248 = -t236 * t160 / f64x8::splat(12.0) - t117 * t160 / f64x8::splat(12.0) - f64x8::splat(0.0625) * t242 * t160 - t127 * t125 * t55 / f64x8::splat(3.0);
            let t249 = f64x8::splat(1.0) / t132;
            let t251 = t248 * t249 * t134;
            let t253 = t201 * t9 * t149 * t103 / f64x8::splat(6.0) + t208 * t224 - t153 - t176 + t227 * t228 * t135 / f64x8::splat(6.0) + t234 * t251;
            let t254 = t74 * t253;
            let t255 = t54 * t254;
            let t257 = t197 * t105 * t143;
            let t258 = f64x8::splat(2.0) * t257;
            let t259 = t226 * t3;
            let t260 = t74 * t259;
            let t263 = t9 * t149 * t133 * t134;
            let t264 = t260 * t263;
            let t265 = t264 / f64x8::splat(6.0);
            let t266 = t74 * t110;
            let t268 = t249 * t134;
            let t269 = t233 * t248 * t268;
            let t270 = t266 * t269;
            let tvrho0 = -t47 + t140 + t145 + t10 * (t153 + t176 + t180 - t185 + t199 + t255 + t258 - t265 - t270);
            acc_vrho_0 = tvrho0;
            let t273 = -t55 - t187;
            let t276 = ((t58).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t61 * t273));
            let t277 = -t273;
            let t280 = ((t65).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t66 * t277));
            let t282 = (t276 + t280) * t73;
            let t283 = t282 * t138;
            let t284 = t54 * t283;
            let t286 = t282 * t105 * t143;
            let t287 = f64x8::splat(2.0) * t286;
            let tvrho1 = -t47 + t140 + t145 + t10 * (t153 + t176 - t180 - t185 + t284 + t255 + t287 - t265 - t270);
            acc_vrho_1 = tvrho1;
            let t290 = t152 / f64x8::splat(3.0);
            let t291 = f64x8::splat(2.0) * t176;
            let t292 = f64x8::splat(8.0) * t179;
            let t293 = f64x8::splat(8.0) * t184;
            let t295 = f64x8::splat(2.0) * t255;
            let t297 = t264 / f64x8::splat(3.0);
            let t298 = f64x8::splat(2.0) * t270;
            let t300 = f64x8::splat(1.0) / t23 / t22;
            let t302 = t3 * t3;
            let t303 = t19 * t300 * t302;
            let t304 = t6 * t6;
            let t305 = t304 * t7;
            let t306 = t11 * t11;
            let t309 = t305 / t306 / t51;
            let t313 = f64x8::splat(1.0) / t11 / t51;
            let t314 = t9 * t313;
            let t319 = f64x8::splat(1.0)/((t22).sqrt());
            let t321 = t30 * t319 * t302;
            let t326 = t37 * t37;
            let t333 = -t303 * t309 / f64x8::splat(18.0) + t159 * t314 / f64x8::splat(9.0) + t27 * t314 / f64x8::splat(9.0) + f64x8::splat(0.041666666666666664) * t321 * t309 + f64x8::splat(0.08333333333333333) * t167 * t314 + t39 * t326 * t186 / f64x8::splat(9.0) + t39 * t37 * t186 / f64x8::splat(3.0);
            let t334 = t333 * t174;
            let t335 = t156 * t334;
            let t336 = t49 * t53;
            let t337 = t336 * t139;
            let t338 = f64x8::splat(12.0) * t337;
            let t339 = t177 * t182;
            let t340 = t339 * t139;
            let t341 = f64x8::splat(32.0) * t340;
            let t343 = f64x8::splat(1.0) / t52 / t51;
            let t344 = t50 * t343;
            let t345 = t344 * t139;
            let t346 = f64x8::splat(20.0) * t345;
            let t347 = t154 * t40;
            let t348 = f64x8::splat(1.0) / t347;
            let t349 = t16 * t348;
            let t350 = t173 * t173;
            let t351 = t350 * t174;
            let t352 = t349 * t351;
            let t353 = f64x8::splat(2.0) * t352;
            let t354 = t154 * t154;
            let t355 = f64x8::splat(1.0) / t354;
            let t356 = t16 * t355;
            let t357 = t44 * t44;
            let t358 = f64x8::splat(1.0) / t357;
            let t360 = t350 * t358 * t18;
            let t361 = t356 * t360;
            let t362 = t361 / f64x8::splat(2.0);
            let t363 = t178 * t198;
            let t364 = f64x8::splat(8.0) * t363;
            let t365 = t178 * t254;
            let t366 = f64x8::splat(8.0) * t365;
            let t367 = t183 * t198;
            let t368 = f64x8::splat(8.0) * t367;
            let t369 = t183 * t254;
            let t370 = f64x8::splat(8.0) * t369;
            let t371 = t61 * t61;
            let t372 = f64x8::splat(1.0) / t371;
            let t373 = t188 * t188;
            let t376 = t51 * t10;
            let t377 = f64x8::splat(1.0) / t376;
            let t378 = t48 * t377;
            let t380 = -f64x8::splat(2.0) * t186 + f64x8::splat(2.0) * t378;
            let t384 = ((t58).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t372 * t373 + f64x8::splat(4.0) / f64x8::splat(3.0) * t61 * t380));
            let t385 = t66 * t66;
            let t386 = f64x8::splat(1.0) / t385;
            let t387 = t192 * t192;
            let t390 = -t380;
            let t394 = ((t65).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t386 * t387 + f64x8::splat(4.0) / f64x8::splat(3.0) * t66 * t390));
            let t396 = (t384 + t394) * t73;
            let t397 = t396 * t138;
            let t398 = t54 * t397;
            let t399 = t335 + t338 - t341 + t346 - t353 + t362 + t364 + t366 - t368 - t370 + t398;
            let t400 = t197 * t253;
            let t401 = t54 * t400;
            let t402 = f64x8::splat(2.0) * t401;
            let t407 = t77 * t9;
            let t408 = t149 * t207;
            let t412 = t206 * t98;
            let t413 = f64x8::splat(1.0) / t412;
            let t414 = t80 * t413;
            let t415 = t222 * t222;
            let t416 = t415 * t223;
            let t420 = t83 * t300 * t302;
            let t428 = t90 * t319 * t302;
            let t433 = t95 * t95;
            let t440 = -t420 * t309 / f64x8::splat(18.0) + t210 * t314 / f64x8::splat(9.0) + t87 * t314 / f64x8::splat(9.0) + f64x8::splat(0.041666666666666664) * t428 * t309 + f64x8::splat(0.08333333333333333) * t216 * t314 + t97 * t433 * t186 / f64x8::splat(9.0) + t97 * t95 * t186 / f64x8::splat(3.0);
            let t441 = t440 * t223;
            let t443 = t206 * t206;
            let t444 = f64x8::splat(1.0) / t443;
            let t445 = t80 * t444;
            let t446 = t102 * t102;
            let t447 = f64x8::splat(1.0) / t446;
            let t449 = t415 * t447 * t82;
            let t454 = t147 * t9 * t313 * t45;
            let t455 = f64x8::splat(2.0) / f64x8::splat(9.0) * t454;
            let t456 = t4 * t9;
            let t457 = t149 * t155;
            let t459 = t456 * t457 * t175;
            let t460 = t459 / f64x8::splat(6.0);
            let t461 = t8 * t313;
            let t465 = t107 * t9;
            let t466 = t149 * t233;
            let t470 = t232 * t128;
            let t471 = f64x8::splat(1.0) / t470;
            let t472 = t110 * t471;
            let t473 = t248 * t248;
            let t474 = t473 * t249;
            let t475 = t474 * t134;
            let t479 = t113 * t300 * t302;
            let t487 = t120 * t319 * t302;
            let t492 = t125 * t125;
            let t499 = -t479 * t309 / f64x8::splat(18.0) + t236 * t314 / f64x8::splat(9.0) + t117 * t314 / f64x8::splat(9.0) + f64x8::splat(0.041666666666666664) * t487 * t309 + f64x8::splat(0.08333333333333333) * t242 * t314 + t127 * t492 * t186 / f64x8::splat(9.0) + t127 * t125 * t186 / f64x8::splat(3.0);
            let t500 = t499 * t249;
            let t501 = t500 * t134;
            let t503 = t232 * t232;
            let t504 = f64x8::splat(1.0) / t503;
            let t505 = t110 * t504;
            let t506 = t505 * t473;
            let t507 = t132 * t132;
            let t508 = f64x8::splat(1.0) / t507;
            let t509 = t508 * t134;
            let t510 = t509 * t112;
            let t513 = -f64x8::splat(2.0) / f64x8::splat(9.0) * t201 * t9 * t313 * t103 - t407 * t408 * t224 / f64x8::splat(6.0) - f64x8::splat(2.0) * t414 * t416 + t208 * t441 + t445 * t449 / f64x8::splat(2.0) + t455 + t460 + t353 - t335 - t362 - f64x8::splat(2.0) / f64x8::splat(9.0) * t227 * t461 * t135 - t465 * t466 * t251 / f64x8::splat(6.0) - f64x8::splat(2.0) * t472 * t475 + t234 * t501 + t506 * t510 / f64x8::splat(2.0);
            let t514 = t74 * t513;
            let t515 = t54 * t514;
            let t516 = t197 * t110;
            let t517 = t516 * t269;
            let t518 = f64x8::splat(2.0) * t517;
            let t520 = t233 * t499 * t268;
            let t521 = t266 * t520;
            let t522 = t197 * t259;
            let t523 = t522 * t263;
            let t524 = t523 / f64x8::splat(3.0);
            let t526 = t471 * t473 * t268;
            let t527 = t266 * t526;
            let t528 = f64x8::splat(2.0) * t527;
            let t529 = t74 * t505;
            let t530 = t473 * t508;
            let t531 = t134 * t112;
            let t532 = t530 * t531;
            let t533 = t529 * t532;
            let t534 = t533 / f64x8::splat(2.0);
            let t536 = t396 * t105 * t143;
            let t537 = f64x8::splat(2.0) * t536;
            let t540 = t9 * t313 * t133 * t134;
            let t541 = t260 * t540;
            let t542 = f64x8::splat(2.0) / f64x8::splat(9.0) * t541;
            let t543 = t107 * t6;
            let t544 = t74 * t543;
            let t545 = t228 * t233;
            let t546 = t545 * t251;
            let t547 = t544 * t546;
            let t548 = t547 / f64x8::splat(6.0);
            let t549 = t402 + t515 - t518 - t521 - t524 - t455 - t460 + t528 - t534 + t537 + t542 + t548;
            let tv2rho20 = t290 + t291 + t292 - t293 + f64x8::splat(2.0) * t199 + t295 + f64x8::splat(4.0) * t257 - t297 - t298 + t10 * (t399 + t549);
            acc_v2rho2_0 = tv2rho20;
            let t552 = t372 * t273;
            let t555 = t61 * t48;
            let t559 = ((t58).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t552 * t188 + f64x8::splat(8.0) / f64x8::splat(3.0) * t555 * t377));
            let t560 = t386 * t277;
            let t563 = t66 * t48;
            let t567 = ((t65).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t560 * t192 - f64x8::splat(8.0) / f64x8::splat(3.0) * t563 * t377));
            let t569 = (t559 + t567) * t73;
            let t571 = t569 * t105 * t143;
            let t573 = t282 * t259;
            let t574 = t573 * t263;
            let t577 = t569 * t138;
            let t578 = t54 * t577;
            let t579 = t282 * t253;
            let t580 = t54 * t579;
            let t581 = t178 * t283;
            let t583 = t548 + f64x8::splat(2.0) * t571 - t353 + t335 - t460 - t534 - t574 / f64x8::splat(6.0) - t523 / f64x8::splat(6.0) + t542 - t455 + t578 + t580 + f64x8::splat(4.0) * t581;
            let t584 = t183 * t283;
            let t588 = t282 * t110;
            let t589 = t588 * t269;
            let t590 = -f64x8::splat(4.0) * t584 - t338 + t346 + t362 - f64x8::splat(4.0) * t363 - f64x8::splat(4.0) * t367 - t517 - t521 + t528 - t589 - t370 + t401 + t515;
            let tv2rho21 = t290 + t291 - t293 + t199 + t295 + t258 - t297 - t298 + t284 + t287 + t10 * (t583 + t590);
            acc_v2rho2_1 = tv2rho21;
            let t595 = t273 * t273;
            let t599 = f64x8::splat(2.0) * t186 + f64x8::splat(2.0) * t378;
            let t603 = ((t58).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t372 * t595 + f64x8::splat(4.0) / f64x8::splat(3.0) * t61 * t599));
            let t604 = t277 * t277;
            let t607 = -t599;
            let t611 = ((t65).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t386 * t604 + f64x8::splat(4.0) / f64x8::splat(3.0) * t66 * t607));
            let t613 = (t603 + t611) * t73;
            let t614 = t613 * t138;
            let t615 = t54 * t614;
            let t617 = t613 * t105 * t143;
            let t618 = f64x8::splat(2.0) * t617;
            let t619 = t615 + t618 + t335 + t338 + t341 + t346 - t353 + t362 - t366 - t370 + t515;
            let t620 = f64x8::splat(2.0) * t589;
            let t621 = t574 / f64x8::splat(3.0);
            let t622 = f64x8::splat(2.0) * t580;
            let t623 = f64x8::splat(8.0) * t581;
            let t624 = f64x8::splat(8.0) * t584;
            let t625 = -t620 - t521 - t621 - t455 - t460 + t528 - t534 + t622 - t623 - t624 + t542 + t548;
            let tv2rho22 = t290 + t291 - t292 - t293 + f64x8::splat(2.0) * t284 + t295 + f64x8::splat(4.0) * t286 - t297 - t298 + t10 * (t619 + t625);
            acc_v2rho2_2 = tv2rho22;
            let t628 = f64x8::splat(3.0) * t335;
            let t630 = t20 * t8;
            let t631 = t74 * t106 * t630;
            let t632 = t149 * t504;
            let t634 = t632 * t473 * t510;
            let t635 = t631 * t634;
            let t636 = t635 / f64x8::splat(8.0);
            let t637 = t197 * t543;
            let t638 = t637 * t546;
            let t639 = t638 / f64x8::splat(2.0);
            let t640 = t545 * t501;
            let t641 = t544 * t640;
            let t642 = t641 / f64x8::splat(4.0);
            let t644 = t228 * t471 * t475;
            let t645 = t544 * t644;
            let t646 = t645 / f64x8::splat(2.0);
            let t647 = t461 * t233;
            let t648 = t647 * t251;
            let t649 = t544 * t648;
            let t650 = t649 / f64x8::splat(3.0);
            let t652 = f64x8::splat(1.0) / t503 / t128;
            let t653 = t110 * t652;
            let t654 = t74 * t653;
            let t655 = t473 * t248;
            let t657 = t655 * t508 * t531;
            let t658 = t654 * t657;
            let t659 = f64x8::splat(3.0) * t658;
            let t660 = t197 * t505;
            let t661 = t660 * t532;
            let t662 = f64x8::splat(3.0) / f64x8::splat(2.0) * t661;
            let t663 = t74 * t472;
            let t664 = t134 * t248;
            let t665 = t500 * t664;
            let t666 = t663 * t665;
            let t667 = f64x8::splat(6.0) * t666;
            let t669 = f64x8::splat(1.0) / t503 / t232;
            let t670 = t110 * t669;
            let t671 = t74 * t670;
            let t673 = f64x8::splat(1.0) / t507 / t132;
            let t675 = t105 * t105;
            let t676 = f64x8::splat(1.0) / t675;
            let t677 = t134 * t676;
            let t678 = t655 * t673 * t677;
            let t679 = t671 * t678;
            let t680 = t679 / f64x8::splat(2.0);
            let t681 = t313 * t155;
            let t683 = t456 * t681 * t175;
            let t684 = t683 / f64x8::splat(3.0);
            let t685 = t149 * t348;
            let t687 = t456 * t685 * t351;
            let t688 = t687 / f64x8::splat(2.0);
            let t690 = t456 * t457 * t334;
            let t691 = t690 / f64x8::splat(4.0);
            let t692 = t636 + t639 + t642 - t646 - t650 + t659 - t662 + t667 - t680 + t684 + t688 - t691;
            let t693 = t522 * t540;
            let t694 = f64x8::splat(2.0) / f64x8::splat(3.0) * t693;
            let t696 = f64x8::splat(1.0) / t11 / t376;
            let t699 = t9 * t696 * t133 * t134;
            let t700 = t260 * t699;
            let t701 = f64x8::splat(14.0) / f64x8::splat(27.0) * t700;
            let t702 = t396 * t259;
            let t703 = t702 * t263;
            let t711 = f64x8::splat(1.0) / t23 / t302 / t304 / t7 * t306 / f64x8::splat(4.0);
            let t712 = t19 * t711;
            let t713 = t5 * t53;
            let t718 = t305 / t306 / t376;
            let t721 = t9 * t696;
            let t726 = f64x8::splat(1.0)/((t22) * (t22).sqrt());
            let t727 = t30 * t726;
            let t734 = t326 * t37;
            let t744 = -t712 * t713 / f64x8::splat(3.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t303 * t718 - f64x8::splat(7.0) / f64x8::splat(27.0) * t159 * t721 - f64x8::splat(7.0) / f64x8::splat(27.0) * t27 * t721 + f64x8::splat(0.08333333333333333) * t727 * t713 - f64x8::splat(0.16666666666666666) * t321 * t718 - f64x8::splat(0.19444444444444445) * t167 * t721 - t39 * t734 * t377 / f64x8::splat(27.0) - t39 * t326 * t377 / f64x8::splat(3.0) - f64x8::splat(2.0) / f64x8::splat(3.0) * t39 * t37 * t377;
            let t745 = t744 * t174;
            let t746 = t156 * t745;
            let t747 = t350 * t173;
            let t748 = t747 * t174;
            let t749 = t356 * t748;
            let t750 = f64x8::splat(6.0) * t749;
            let t752 = t504 * t655 * t268;
            let t753 = t266 * t752;
            let t754 = f64x8::splat(6.0) * t753;
            let t755 = t516 * t526;
            let t756 = f64x8::splat(6.0) * t755;
            let t758 = t358 * t18;
            let t759 = t758 * t173;
            let t760 = t356 * t333 * t759;
            let t761 = f64x8::splat(3.0) / f64x8::splat(2.0) * t760;
            let t763 = f64x8::splat(1.0) / t52 / t376;
            let t764 = t50 * t763;
            let t765 = t764 * t139;
            let t766 = f64x8::splat(120.0) * t765;
            let t767 = t177 * t343;
            let t768 = t767 * t139;
            let t769 = f64x8::splat(240.0) * t768;
            let t770 = t48 * t53;
            let t771 = t770 * t139;
            let t772 = f64x8::splat(24.0) * t771;
            let t773 = t49 * t182;
            let t774 = t773 * t139;
            let t775 = f64x8::splat(144.0) * t774;
            let t776 = t339 * t254;
            let t777 = f64x8::splat(96.0) * t776;
            let t778 = t694 - t701 - t703 / f64x8::splat(2.0) + t746 + t750 - t754 + t756 + t761 - t766 + t769 + t772 - t775 - t777;
            let t780 = t336 * t198;
            let t781 = f64x8::splat(36.0) * t780;
            let t782 = t336 * t254;
            let t783 = f64x8::splat(36.0) * t782;
            let t784 = t344 * t254;
            let t785 = f64x8::splat(60.0) * t784;
            let t786 = t339 * t198;
            let t787 = f64x8::splat(96.0) * t786;
            let t788 = t178 * t400;
            let t789 = f64x8::splat(24.0) * t788;
            let t790 = t344 * t198;
            let t791 = f64x8::splat(60.0) * t790;
            let t792 = t178 * t397;
            let t796 = t147 * t9 * t696 * t45;
            let t797 = f64x8::splat(14.0) / f64x8::splat(27.0) * t796;
            let t798 = t178 * t514;
            let t799 = f64x8::splat(12.0) * t798;
            let t800 = t183 * t400;
            let t801 = f64x8::splat(24.0) * t800;
            let t802 = t183 * t514;
            let t803 = f64x8::splat(12.0) * t802;
            let t804 = t183 * t397;
            let t806 = t781 + t783 + t785 - t787 + t789 + t791 + f64x8::splat(12.0) * t792 + t797 + t799 - t801 - t803 - f64x8::splat(12.0) * t804;
            let t807 = t396 * t253;
            let t808 = t54 * t807;
            let t810 = t197 * t513;
            let t811 = t54 * t810;
            let t812 = f64x8::splat(3.0) * t811;
            let t814 = f64x8::splat(1.0) / t371 / t57;
            let t815 = t373 * t188;
            let t818 = t372 * t188;
            let t822 = f64x8::splat(6.0) * t377 - f64x8::splat(6.0) * t770;
            let t826 = ((t58).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t814 * t815 + f64x8::splat(4.0) / f64x8::splat(3.0) * t818 * t380 + f64x8::splat(4.0) / f64x8::splat(3.0) * t61 * t822));
            let t828 = f64x8::splat(1.0) / t385 / t64;
            let t829 = t387 * t192;
            let t832 = t386 * t192;
            let t835 = -t822;
            let t839 = ((t65).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t828 * t829 + f64x8::splat(4.0) / f64x8::splat(3.0) * t832 * t390 + f64x8::splat(4.0) / f64x8::splat(3.0) * t66 * t835));
            let t841 = (t826 + t839) * t73;
            let t842 = t841 * t138;
            let t843 = t54 * t842;
            let t845 = t841 * t105 * t143;
            let t846 = f64x8::splat(2.0) * t845;
            let t847 = t113 * t711;
            let t856 = t120 * t726;
            let t863 = t492 * t125;
            let t873 = -t847 * t713 / f64x8::splat(3.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t479 * t718 - f64x8::splat(7.0) / f64x8::splat(27.0) * t236 * t721 - f64x8::splat(7.0) / f64x8::splat(27.0) * t117 * t721 + f64x8::splat(0.08333333333333333) * t856 * t713 - f64x8::splat(0.16666666666666666) * t487 * t718 - f64x8::splat(0.19444444444444445) * t242 * t721 - t127 * t863 * t377 / f64x8::splat(27.0) - t127 * t492 * t377 / f64x8::splat(3.0) - f64x8::splat(2.0) / f64x8::splat(3.0) * t127 * t125 * t377;
            let t875 = t233 * t873 * t268;
            let t876 = t266 * t875;
            let t877 = t396 * t110;
            let t878 = t877 * t269;
            let t880 = t516 * t520;
            let t881 = f64x8::splat(3.0) * t880;
            let t882 = t149 * t355;
            let t884 = t456 * t882 * t360;
            let t885 = t884 / f64x8::splat(8.0);
            let t886 = t83 * t711;
            let t895 = t90 * t726;
            let t902 = t433 * t95;
            let t912 = -t886 * t713 / f64x8::splat(3.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t420 * t718 - f64x8::splat(7.0) / f64x8::splat(27.0) * t210 * t721 - f64x8::splat(7.0) / f64x8::splat(27.0) * t87 * t721 + f64x8::splat(0.08333333333333333) * t895 * t713 - f64x8::splat(0.16666666666666666) * t428 * t718 - f64x8::splat(0.19444444444444445) * t216 * t721 - t97 * t902 * t377 / f64x8::splat(27.0) - t97 * t433 * t377 / f64x8::splat(3.0) - f64x8::splat(2.0) / f64x8::splat(3.0) * t97 * t95 * t377;
            let t913 = t912 * t223;
            let t915 = t415 * t222;
            let t916 = t915 * t223;
            let t919 = t313 * t207;
            let t926 = t149 * t413;
            let t930 = t8 * t696;
            let t934 = t107 * t160;
            let t936 = t504 * t473 * t510;
            let t945 = t509 * t112 * t248;
            let t948 = t208 * t913 + f64x8::splat(6.0) * t445 * t916 - t684 + t407 * t919 * t224 / f64x8::splat(3.0) - t407 * t408 * t441 / f64x8::splat(4.0) - t688 + t691 + t407 * t926 * t416 / f64x8::splat(2.0) + f64x8::splat(14.0) / f64x8::splat(27.0) * t227 * t930 * t135 - t934 * t936 / f64x8::splat(8.0) - t746 - t750 - t761 - t797 + f64x8::splat(14.0) / f64x8::splat(27.0) * t201 * t9 * t696 * t103 + f64x8::splat(3.0) / f64x8::splat(2.0) * t505 * t499 * t945;
            let t950 = t447 * t82;
            let t951 = t950 * t222;
            let t954 = t472 * t248;
            let t958 = t673 * t134;
            let t959 = t958 * t676;
            let t965 = t149 * t471;
            let t969 = t149 * t444;
            let t974 = t655 * t249 * t134;
            let t977 = t873 * t249;
            let t978 = t977 * t134;
            let t981 = f64x8::splat(1.0) / t443 / t206;
            let t982 = t80 * t981;
            let t984 = f64x8::splat(1.0) / t446 / t102;
            let t986 = t75 * t75;
            let t987 = f64x8::splat(1.0) / t986;
            let t988 = t915 * t984 * t987;
            let t992 = f64x8::splat(1.0) / t443 / t98;
            let t993 = t80 * t992;
            let t995 = t915 * t447 * t82;
            let t998 = t224 * t440;
            let t1002 = f64x8::splat(1.0) / t354 / t154;
            let t1003 = t16 * t1002;
            let t1005 = f64x8::splat(1.0) / t357 / t44;
            let t1007 = t1 * t1;
            let t1008 = f64x8::splat(1.0) / t1007;
            let t1009 = t747 * t1005 * t1008;
            let t1010 = t1003 * t1009;
            let t1011 = t1010 / f64x8::splat(2.0);
            let t1013 = f64x8::splat(1.0) / t354 / t40;
            let t1014 = t16 * t1013;
            let t1016 = t747 * t358 * t18;
            let t1017 = t1014 * t1016;
            let t1018 = f64x8::splat(3.0) * t1017;
            let t1019 = t334 * t173;
            let t1020 = t349 * t1019;
            let t1021 = f64x8::splat(6.0) * t1020;
            let t1022 = t313 * t233;
            let t1029 = f64x8::splat(3.0) / f64x8::splat(2.0) * t445 * t440 * t951 - f64x8::splat(6.0) * t954 * t501 + t670 * t655 * t959 / f64x8::splat(2.0) - f64x8::splat(3.0) * t653 * t655 * t510 + t465 * t965 * t475 / f64x8::splat(2.0) + t885 - t407 * t969 * t449 / f64x8::splat(8.0) + f64x8::splat(6.0) * t505 * t974 + t234 * t978 + t982 * t988 / f64x8::splat(2.0) - f64x8::splat(3.0) * t993 * t995 - f64x8::splat(6.0) * t414 * t998 - t1011 + t1018 + t1021 + t465 * t1022 * t251 / f64x8::splat(3.0) - t465 * t466 * t501 / f64x8::splat(4.0);
            let t1030 = t948 + t1029;
            let t1031 = t74 * t1030;
            let t1032 = t54 * t1031;
            let t1034 = t531 * t248;
            let t1035 = t499 * t508 * t1034;
            let t1036 = t529 * t1035;
            let t1037 = f64x8::splat(3.0) / f64x8::splat(2.0) * t1036;
            let t1038 = f64x8::splat(3.0) * t808 + t812 + t843 + t846 - t876 - f64x8::splat(3.0) * t878 - t881 - t885 + t1032 + t1011 - t1018 - t1021 - t1037;
            let t1042 = f64x8::splat(36.0) * t337;
            let t1043 = f64x8::splat(96.0) * t340;
            let t1044 = f64x8::splat(60.0) * t345;
            let t1045 = f64x8::splat(6.0) * t352;
            let t1046 = f64x8::splat(3.0) / f64x8::splat(2.0) * t361;
            let t1048 = f64x8::splat(24.0) * t365;
            let t1050 = f64x8::splat(24.0) * t369;
            let t1052 = t628 + t10 * (t692 + t778 + t806 + t1038) + t1042 - t1043 + t1044 - t1045 + t1046 + f64x8::splat(24.0) * t363 + t1048 - f64x8::splat(24.0) * t367 - t1050 + f64x8::splat(3.0) * t398;
            let t1054 = f64x8::splat(3.0) * t515;
            let t1056 = f64x8::splat(3.0) * t521;
            let t1057 = f64x8::splat(2.0) / f64x8::splat(3.0) * t454;
            let t1058 = t459 / f64x8::splat(2.0);
            let t1059 = f64x8::splat(6.0) * t527;
            let t1060 = f64x8::splat(3.0) / f64x8::splat(2.0) * t533;
            let t1062 = f64x8::splat(2.0) / f64x8::splat(3.0) * t541;
            let t1063 = t547 / f64x8::splat(2.0);
            let t1064 = f64x8::splat(6.0) * t401 + t1054 - f64x8::splat(6.0) * t517 - t1056 - t523 - t1057 - t1058 + t1059 - t1060 + f64x8::splat(6.0) * t536 + t1062 + t1063;
            let tv3rho30 = t1052 + t1064;
            acc_v3rho3_0 = tv3rho30;
            let t1065 = t282 * t543;
            let t1066 = t1065 * t546;
            let t1069 = t282 * t505;
            let t1070 = t1069 * t532;
            let t1072 = t573 * t540;
            let t1075 = t636 + t1066 / f64x8::splat(6.0) + t638 / f64x8::splat(3.0) + t642 - t646 - t650 - t1070 / f64x8::splat(2.0) + t659 - t661 + t667 - t680 + t684 + t688 - t691 + f64x8::splat(2.0) / f64x8::splat(9.0) * t1072 + f64x8::splat(4.0) / f64x8::splat(9.0) * t693;
            let t1076 = t569 * t259;
            let t1077 = t1076 * t263;
            let t1078 = t1077 / f64x8::splat(3.0);
            let t1080 = t814 * t273;
            let t1083 = t372 * t48;
            let t1094 = ((t58).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t1080 * t373 + f64x8::splat(16.0) / f64x8::splat(9.0) * t1083 * t377 * t188 + f64x8::splat(4.0) / f64x8::splat(9.0) * t552 * t380 + f64x8::splat(8.0) / f64x8::splat(3.0) * t61 * t377 - f64x8::splat(8.0) * t555 * t53));
            let t1095 = t828 * t277;
            let t1098 = t386 * t48;
            let t1109 = ((t65).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t1095 * t387 - f64x8::splat(16.0) / f64x8::splat(9.0) * t1098 * t377 * t192 + f64x8::splat(4.0) / f64x8::splat(9.0) * t560 * t390 - f64x8::splat(8.0) / f64x8::splat(3.0) * t66 * t377 + f64x8::splat(8.0) * t563 * t53));
            let t1111 = (t1094 + t1109) * t73;
            let t1113 = t1111 * t105 * t143;
            let t1115 = t588 * t520;
            let t1116 = t588 * t526;
            let t1118 = t569 * t110;
            let t1119 = t1118 * t269;
            let t1120 = f64x8::splat(2.0) * t1119;
            let t1121 = t183 * t577;
            let t1122 = f64x8::splat(8.0) * t1121;
            let t1123 = t1111 * t138;
            let t1124 = t54 * t1123;
            let t1125 = t569 * t253;
            let t1126 = t54 * t1125;
            let t1127 = f64x8::splat(2.0) * t1126;
            let t1128 = t178 * t579;
            let t1130 = t183 * t579;
            let t1132 = t178 * t577;
            let t1133 = f64x8::splat(8.0) * t1132;
            let t1134 = t344 * t283;
            let t1136 = -t701 - t1078 - t703 / f64x8::splat(6.0) + t746 + t750 + f64x8::splat(2.0) * t1113 - t1115 + f64x8::splat(2.0) * t1116 - t1120 - t1122 + t1124 + t1127 + f64x8::splat(8.0) * t1128 - f64x8::splat(8.0) * t1130 + t1133 + f64x8::splat(20.0) * t1134;
            let t1138 = t336 * t283;
            let t1140 = t339 * t283;
            let t1142 = t282 * t513;
            let t1143 = t54 * t1142;
            let t1145 = f64x8::splat(80.0) * t768;
            let t1146 = f64x8::splat(48.0) * t774;
            let t1147 = f64x8::splat(32.0) * t776;
            let t1148 = f64x8::splat(24.0) * t780;
            let t1149 = f64x8::splat(12.0) * t782;
            let t1152 = f64x8::splat(12.0) * t1138 - f64x8::splat(32.0) * t1140 + t1143 - t754 + f64x8::splat(4.0) * t755 + t761 - t766 + t1145 - t772 + t1146 - t1147 - t1148 - t1149 + t785 + f64x8::splat(40.0) * t790 - f64x8::splat(4.0) * t792;
            let t1153 = f64x8::splat(4.0) * t798;
            let t1158 = t797 + t1153 - f64x8::splat(16.0) * t800 - t803 - f64x8::splat(4.0) * t804 + t808 + f64x8::splat(2.0) * t811 - t876 - t878 - f64x8::splat(2.0) * t880 - t885 + t1032 + t1011 - t1018 - t1021 - t1037;
            let t1162 = f64x8::splat(4.0) * t571;
            let t1164 = f64x8::splat(2.0) * t578;
            let t1165 = t10 * (t1075 + t1136 + t1152 + t1158) + t1063 + t1162 + t537 - t1045 + t628 - t1058 - t1060 - t621 - f64x8::splat(2.0) / f64x8::splat(3.0) * t523 + t1062 - t1057 + t1164 + t622 + t623;
            let t1169 = -t624 - t338 - t341 + t1044 + t1046 + t366 - f64x8::splat(16.0) * t367 - f64x8::splat(4.0) * t517 - t1056 + t1059 - t620 - t1050 + t398 + f64x8::splat(4.0) * t401 + t1054;
            let tv3rho31 = t1165 + t1169;
            acc_v3rho3_1 = tv3rho31;
            let t1171 = t814 * t595;
            let t1176 = t372 * t599;
            let t1181 = -f64x8::splat(2.0) * t377 - f64x8::splat(6.0) * t770;
            let t1185 = ((t58).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t1171 * t188 + f64x8::splat(16.0) / f64x8::splat(9.0) * t552 * t378 + f64x8::splat(4.0) / f64x8::splat(9.0) * t1176 * t188 + f64x8::splat(4.0) / f64x8::splat(3.0) * t61 * t1181));
            let t1186 = t828 * t604;
            let t1191 = t386 * t607;
            let t1194 = -t1181;
            let t1198 = ((t65).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t1186 * t192 - f64x8::splat(16.0) / f64x8::splat(9.0) * t560 * t378 + f64x8::splat(4.0) / f64x8::splat(9.0) * t1191 * t192 + f64x8::splat(4.0) / f64x8::splat(3.0) * t66 * t1194));
            let t1200 = (t1185 + t1198) * t73;
            let t1202 = t1200 * t105 * t143;
            let t1206 = t178 * t614;
            let t1208 = t183 * t614;
            let t1210 = t613 * t253;
            let t1211 = t54 * t1210;
            let t1212 = t1200 * t138;
            let t1213 = t54 * t1212;
            let t1215 = t636 + f64x8::splat(2.0) * t1202 + t1066 / f64x8::splat(3.0) + t638 / f64x8::splat(6.0) + t642 - t646 - t650 + f64x8::splat(4.0) * t1206 - f64x8::splat(4.0) * t1208 + t1211 + t1213 - t1070 + t659 - t661 / f64x8::splat(2.0) + t667 - t680;
            let t1217 = t613 * t259;
            let t1218 = t1217 * t263;
            let t1224 = t684 + t688 - t691 + f64x8::splat(4.0) / f64x8::splat(9.0) * t1072 - t1218 / f64x8::splat(6.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t693 - t701 - t1078 + t746 + t750 - f64x8::splat(2.0) * t1115 + f64x8::splat(4.0) * t1116 - t1120 - t1122 + t1127 - f64x8::splat(16.0) * t1130;
            let t1227 = f64x8::splat(24.0) * t1138;
            let t1232 = -t1133 + f64x8::splat(40.0) * t1134 - t1227 + f64x8::splat(2.0) * t1143 - t754 + f64x8::splat(2.0) * t755 + t761 - t766 - t1145 + t772 + t1146 + t1147 + f64x8::splat(12.0) * t780 - t1149 + t785 + f64x8::splat(32.0) * t786;
            let t1236 = t613 * t110;
            let t1237 = t1236 * t269;
            let t1238 = -f64x8::splat(8.0) * t788 + f64x8::splat(20.0) * t790 + t797 - t1153 - f64x8::splat(8.0) * t800 - t803 + t811 - t876 - t880 - t1237 - t885 + t1032 + t1011 - t1018 - t1021 - t1037;
            let t1244 = t1063 + t1162 - t1045 + t628 - t1058 - t1060 - f64x8::splat(2.0) / f64x8::splat(3.0) * t574 - t524 + t1062 - t1057 + t10 * (t1215 + t1224 + t1232 + t1238) + t1164 + f64x8::splat(4.0) * t580 - f64x8::splat(16.0) * t584 + t615;
            let t1246 = t618 - t338 + t341 + t1044 + t1046 - t364 - t366 - t368 - t518 - t1056 + t1059 - f64x8::splat(4.0) * t589 - t1050 + t402 + t1054;
            let tv3rho32 = t1244 + t1246;
            acc_v3rho3_2 = tv3rho32;
            let t1247 = t1066 / f64x8::splat(2.0);
            let t1251 = f64x8::splat(3.0) / f64x8::splat(2.0) * t1070;
            let t1252 = t636 + t1247 + t642 - t646 - t650 - f64x8::splat(12.0) * t1206 - f64x8::splat(12.0) * t1208 + f64x8::splat(3.0) * t1211 - t1251 + t659 + t667 - t680;
            let t1253 = f64x8::splat(2.0) / f64x8::splat(3.0) * t1072;
            let t1255 = f64x8::splat(3.0) * t1115;
            let t1256 = f64x8::splat(6.0) * t1116;
            let t1257 = f64x8::splat(24.0) * t1128;
            let t1258 = f64x8::splat(24.0) * t1130;
            let t1259 = f64x8::splat(60.0) * t1134;
            let t1260 = t684 + t688 - t691 + t1253 - t1218 / f64x8::splat(2.0) - t701 + t746 + t750 - t1255 + t1256 - t1257 - t1258 + t1259;
            let t1262 = f64x8::splat(36.0) * t1138;
            let t1263 = f64x8::splat(96.0) * t1140;
            let t1264 = f64x8::splat(3.0) * t1143;
            let t1265 = t595 * t273;
            let t1271 = -f64x8::splat(6.0) * t377 - f64x8::splat(6.0) * t770;
            let t1275 = ((t58).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t814 * t1265 + f64x8::splat(4.0) / f64x8::splat(3.0) * t552 * t599 + f64x8::splat(4.0) / f64x8::splat(3.0) * t61 * t1271));
            let t1276 = t604 * t277;
            let t1281 = -t1271;
            let t1285 = ((t65).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t828 * t1276 + f64x8::splat(4.0) / f64x8::splat(3.0) * t560 * t607 + f64x8::splat(4.0) / f64x8::splat(3.0) * t66 * t1281));
            let t1287 = (t1275 + t1285) * t73;
            let t1289 = t1287 * t105 * t143;
            let t1290 = f64x8::splat(2.0) * t1289;
            let t1291 = t1287 * t138;
            let t1292 = t54 * t1291;
            let t1293 = t1262 + t1263 + t1264 - t754 + t1290 + t761 + t1292 - t766 - t769 - t772 - t775 + t777;
            let t1295 = t783 + t785 + t797 - t799 - t803 - t876 - f64x8::splat(3.0) * t1237 - t885 + t1032 + t1011 - t1018 - t1021 - t1037;
            let t1301 = t10 * (t1252 + t1260 + t1293 + t1295) + f64x8::splat(3.0) * t615 + f64x8::splat(6.0) * t617 + t628 + t1042 + t1043 + t1044 - t1045 + t1046 - t1048 - t1050 + t1054;
            let t1306 = -f64x8::splat(6.0) * t589 - t1056 - t574 - t1057 - t1058 + t1059 - t1060 + f64x8::splat(6.0) * t580 - f64x8::splat(24.0) * t581 - f64x8::splat(24.0) * t584 + t1062 + t1063;
            let tv3rho33 = t1301 + t1306;
            acc_v3rho3_3 = tv3rho33;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(v2rho2, ip, m, 3, 0, acc_v2rho2_0);
        store_strided(v2rho2, ip, m, 3, 1, acc_v2rho2_1);
        store_strided(v2rho2, ip, m, 3, 2, acc_v2rho2_2);
        store_strided(v3rho3, ip, m, 4, 0, acc_v3rho3_0);
        store_strided(v3rho3, ip, m, 4, 1, acc_v3rho3_1);
        store_strided(v3rho3, ip, m, 4, 2, acc_v3rho3_2);
        store_strided(v3rho3, ip, m, 4, 3, acc_v3rho3_3);
        ip += 8;
    }
}
