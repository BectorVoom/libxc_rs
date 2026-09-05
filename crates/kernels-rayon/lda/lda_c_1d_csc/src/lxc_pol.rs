//! LDA_C_1D_CSC lxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_1d_csc.c`
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
pub fn lda_c_1d_csc_lxc_pol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    v3rho3: &mut [f64],
    v4rho4: &mut [f64],
    param_para_4: f64,
    param_para_7: f64,
    param_para_9: f64,
    param_para_8: f64,
    param_para_1: f64,
    param_para_5: f64,
    param_para_2: f64,
    param_para_6: f64,
    param_para_3: f64,
    param_para_0: f64,
    param_ferro_4: f64,
    param_ferro_7: f64,
    param_ferro_9: f64,
    param_ferro_8: f64,
    param_ferro_1: f64,
    param_ferro_5: f64,
    param_ferro_2: f64,
    param_ferro_6: f64,
    param_ferro_3: f64,
    param_ferro_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_para_4 = f64x8::splat(param_para_4);
    let param_para_7 = f64x8::splat(param_para_7);
    let param_para_9 = f64x8::splat(param_para_9);
    let param_para_8 = f64x8::splat(param_para_8);
    let param_para_1 = f64x8::splat(param_para_1);
    let param_para_5 = f64x8::splat(param_para_5);
    let param_para_2 = f64x8::splat(param_para_2);
    let param_para_6 = f64x8::splat(param_para_6);
    let param_para_3 = f64x8::splat(param_para_3);
    let param_para_0 = f64x8::splat(param_para_0);
    let param_ferro_4 = f64x8::splat(param_ferro_4);
    let param_ferro_7 = f64x8::splat(param_ferro_7);
    let param_ferro_9 = f64x8::splat(param_ferro_9);
    let param_ferro_8 = f64x8::splat(param_ferro_8);
    let param_ferro_1 = f64x8::splat(param_ferro_1);
    let param_ferro_5 = f64x8::splat(param_ferro_5);
    let param_ferro_2 = f64x8::splat(param_ferro_2);
    let param_ferro_6 = f64x8::splat(param_ferro_6);
    let param_ferro_3 = f64x8::splat(param_ferro_3);
    let param_ferro_0 = f64x8::splat(param_ferro_0);
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
        let mut acc_v4rho4_0 = V_ZERO;
        let mut acc_v4rho4_1 = V_ZERO;
        let mut acc_v4rho4_2 = V_ZERO;
        let mut acc_v4rho4_3 = V_ZERO;
        let mut acc_v4rho4_4 = V_ZERO;
        {
            let t1 = v_rho0 + v_rho1;
            let t2 = f64x8::splat(1.0) / t1;
            let t3 = t2 / f64x8::splat(2.0);
            let t4 = param_para_4;
            let t5 = t1 * t1;
            let t6 = f64x8::splat(1.0) / t5;
            let t9 = t3 + t4 * t6 / f64x8::splat(4.0);
            let t10 = param_para_7;
            let t14 = param_para_9;
            let t15 = (simd::pow(t3, t14));
            let t16 = param_para_8 * t15;
            let t17 = f64x8::splat(1.0) + t10 * t2 / f64x8::splat(2.0) + t16;
            let t18 = (simd::ln(t17));
            let t19 = t9 * t18;
            let t22 = param_para_1;
            let t25 = param_para_5;
            let t26 = (simd::pow(t3, t25));
            let t27 = param_para_2 * t26;
            let t30 = param_para_6;
            let t31 = (simd::pow(t3, t30));
            let t32 = param_para_3 * t31;
            let t34 = t22 * t2 + f64x8::splat(2.0) * t27 + f64x8::splat(2.0) * t32 + f64x8::splat(2.0) * param_para_0;
            let t35 = f64x8::splat(1.0) / t34;
            let t36 = t19 * t35;
            let t37 = param_ferro_4;
            let t40 = t3 + t37 * t6 / f64x8::splat(4.0);
            let t41 = param_ferro_7;
            let t45 = param_ferro_9;
            let t46 = (simd::pow(t3, t45));
            let t47 = param_ferro_8 * t46;
            let t48 = f64x8::splat(1.0) + t41 * t2 / f64x8::splat(2.0) + t47;
            let t49 = (simd::ln(t48));
            let t50 = t40 * t49;
            let t53 = param_ferro_1;
            let t56 = param_ferro_5;
            let t57 = (simd::pow(t3, t56));
            let t58 = param_ferro_2 * t57;
            let t61 = param_ferro_6;
            let t62 = (simd::pow(t3, t61));
            let t63 = param_ferro_3 * t62;
            let t65 = t53 * t2 + f64x8::splat(2.0) * t58 + f64x8::splat(2.0) * t63 + f64x8::splat(2.0) * param_ferro_0;
            let t66 = f64x8::splat(1.0) / t65;
            let t68 = -t50 * t66 + t36;
            let t69 = v_rho0 - v_rho1;
            let t70 = t69 * t69;
            let t71 = t68 * t70;
            let t72 = t71 * t6;
            let tzk0 = -t36 + t72;
            acc_zk = tzk0;
            let t74 = f64x8::splat(1.0) / t5 / t1;
            let t77 = -t4 * t74 / f64x8::splat(2.0) - t6 / f64x8::splat(2.0);
            let t78 = t77 * t18;
            let t79 = t78 * t35;
            let t84 = -t10 * t6 / f64x8::splat(2.0) - t16 * t14 * t2;
            let t85 = t9 * t84;
            let t86 = f64x8::splat(1.0) / t17;
            let t87 = t86 * t35;
            let t88 = t85 * t87;
            let t89 = t34 * t34;
            let t90 = f64x8::splat(1.0) / t89;
            let t98 = -f64x8::splat(2.0) * t27 * t25 * t2 - f64x8::splat(2.0) * t32 * t30 * t2 - t22 * t6;
            let t99 = t90 * t98;
            let t100 = t19 * t99;
            let t103 = -t37 * t74 / f64x8::splat(2.0) - t6 / f64x8::splat(2.0);
            let t104 = t103 * t49;
            let t110 = -t41 * t6 / f64x8::splat(2.0) - t47 * t45 * t2;
            let t111 = t40 * t110;
            let t112 = f64x8::splat(1.0) / t48;
            let t113 = t112 * t66;
            let t115 = t65 * t65;
            let t116 = f64x8::splat(1.0) / t115;
            let t124 = -f64x8::splat(2.0) * t58 * t56 * t2 - f64x8::splat(2.0) * t63 * t61 * t2 - t53 * t6;
            let t125 = t116 * t124;
            let t127 = -t104 * t66 - t111 * t113 + t50 * t125 - t100 + t79 + t88;
            let t128 = t127 * t70;
            let t129 = t128 * t6;
            let t130 = t68 * t69;
            let t131 = t130 * t6;
            let t132 = f64x8::splat(2.0) * t131;
            let t133 = t71 * t74;
            let t134 = f64x8::splat(2.0) * t133;
            let tvrho0 = -t36 + t72 + t1 * (-t79 - t88 + t100 + t129 + t132 - t134);
            acc_vrho_0 = tvrho0;
            let tvrho1 = -t36 + t72 + t1 * (-t79 - t88 + t100 + t129 - t132 - t134);
            acc_vrho_1 = tvrho1;
            let t139 = f64x8::splat(2.0) * t79;
            let t140 = f64x8::splat(2.0) * t88;
            let t141 = f64x8::splat(2.0) * t100;
            let t142 = f64x8::splat(2.0) * t129;
            let t143 = f64x8::splat(4.0) * t131;
            let t144 = f64x8::splat(4.0) * t133;
            let t145 = t5 * t5;
            let t146 = f64x8::splat(1.0) / t145;
            let t149 = t74 + f64x8::splat(3.0) / f64x8::splat(2.0) * t4 * t146;
            let t150 = t149 * t18;
            let t151 = t150 * t35;
            let t152 = t77 * t84;
            let t153 = t152 * t87;
            let t154 = f64x8::splat(2.0) * t153;
            let t155 = t78 * t99;
            let t156 = f64x8::splat(2.0) * t155;
            let t158 = t14 * t14;
            let t163 = t16 * t14 * t6 + t16 * t158 * t6 + t10 * t74;
            let t164 = t9 * t163;
            let t165 = t164 * t87;
            let t166 = t84 * t84;
            let t167 = t9 * t166;
            let t168 = t17 * t17;
            let t169 = f64x8::splat(1.0) / t168;
            let t170 = t169 * t35;
            let t171 = t167 * t170;
            let t172 = t86 * t90;
            let t173 = t172 * t98;
            let t174 = t85 * t173;
            let t175 = f64x8::splat(2.0) * t174;
            let t177 = f64x8::splat(1.0) / t89 / t34;
            let t178 = t98 * t98;
            let t179 = t177 * t178;
            let t180 = t19 * t179;
            let t181 = f64x8::splat(2.0) * t180;
            let t183 = t25 * t25;
            let t188 = t30 * t30;
            let t194 = f64x8::splat(2.0) * t27 * t183 * t6 + f64x8::splat(2.0) * t32 * t188 * t6 + f64x8::splat(2.0) * t27 * t25 * t6 + f64x8::splat(2.0) * t32 * t30 * t6 + f64x8::splat(2.0) * t22 * t74;
            let t195 = t90 * t194;
            let t196 = t19 * t195;
            let t199 = t74 + f64x8::splat(3.0) / f64x8::splat(2.0) * t37 * t146;
            let t200 = t199 * t49;
            let t202 = t103 * t110;
            let t208 = t45 * t45;
            let t213 = t47 * t208 * t6 + t47 * t45 * t6 + t41 * t74;
            let t214 = t40 * t213;
            let t216 = t110 * t110;
            let t217 = t40 * t216;
            let t218 = t48 * t48;
            let t219 = f64x8::splat(1.0) / t218;
            let t220 = t219 * t66;
            let t222 = t112 * t116;
            let t223 = t222 * t124;
            let t227 = f64x8::splat(1.0) / t115 / t65;
            let t228 = t124 * t124;
            let t229 = t227 * t228;
            let t233 = t56 * t56;
            let t238 = t61 * t61;
            let t244 = f64x8::splat(2.0) * t58 * t233 * t6 + f64x8::splat(2.0) * t63 * t238 * t6 + f64x8::splat(2.0) * t58 * t56 * t6 + f64x8::splat(2.0) * t63 * t61 * t6 + f64x8::splat(2.0) * t53 * t74;
            let t245 = t116 * t244;
            let t247 = f64x8::splat(2.0) * t104 * t125 + f64x8::splat(2.0) * t111 * t223 - f64x8::splat(2.0) * t202 * t113 - t214 * t113 - t200 * t66 + t217 * t220 - f64x8::splat(2.0) * t50 * t229 + t50 * t245 + t151 + t154 - t156 + t165 - t171 - t175 + t181 - t196;
            let t248 = t247 * t70;
            let t249 = t248 * t6;
            let t250 = t127 * t69;
            let t251 = t250 * t6;
            let t252 = f64x8::splat(4.0) * t251;
            let t253 = t128 * t74;
            let t254 = f64x8::splat(4.0) * t253;
            let t255 = t68 * t6;
            let t256 = f64x8::splat(2.0) * t255;
            let t257 = t130 * t74;
            let t258 = f64x8::splat(8.0) * t257;
            let t259 = t71 * t146;
            let t260 = f64x8::splat(6.0) * t259;
            let t261 = -t151 - t154 + t156 - t165 + t171 + t175 - t181 + t196 + t249 + t252 - t254 + t256 - t258 + t260;
            let tv2rho20 = t1 * t261 - t139 - t140 + t141 + t142 + t143 - t144;
            acc_v2rho2_0 = tv2rho20;
            let t263 = -t151 - t154 + t156 - t165 + t171 + t175 - t181 + t196 + t249 - t254 - t256 + t260;
            let tv2rho21 = t1 * t263 - t139 - t140 + t141 + t142 - t144;
            acc_v2rho2_1 = tv2rho21;
            let t265 = -t151 - t154 + t156 - t165 + t171 + t175 - t181 + t196 + t249 - t252 - t254 + t256 + t258 + t260;
            let tv2rho22 = t1 * t265 - t139 - t140 + t141 + t142 - t143 - t144;
            acc_v2rho2_2 = tv2rho22;
            let t267 = f64x8::splat(3.0) * t171;
            let t268 = f64x8::splat(6.0) * t174;
            let t269 = f64x8::splat(6.0) * t180;
            let t270 = f64x8::splat(6.0) * t155;
            let t271 = f64x8::splat(6.0) * t153;
            let t272 = f64x8::splat(3.0) * t165;
            let t273 = f64x8::splat(3.0) * t196;
            let t274 = f64x8::splat(12.0) * t251;
            let t275 = f64x8::splat(12.0) * t253;
            let t276 = f64x8::splat(24.0) * t257;
            let t277 = f64x8::splat(18.0) * t259;
            let t278 = f64x8::splat(3.0) * t151;
            let t279 = f64x8::splat(3.0) * t249;
            let t280 = f64x8::splat(6.0) * t255;
            let t281 = t127 * t6;
            let t283 = t68 * t74;
            let t285 = t177 * t98;
            let t286 = t285 * t194;
            let t287 = t19 * t286;
            let t288 = f64x8::splat(6.0) * t287;
            let t290 = t86 * t177 * t178;
            let t291 = t85 * t290;
            let t292 = f64x8::splat(6.0) * t291;
            let t293 = t152 * t173;
            let t294 = f64x8::splat(6.0) * t293;
            let t295 = t170 * t84;
            let t296 = t164 * t295;
            let t297 = f64x8::splat(3.0) * t296;
            let t298 = t164 * t173;
            let t299 = f64x8::splat(3.0) * t298;
            let t300 = t169 * t90;
            let t301 = t300 * t98;
            let t302 = t167 * t301;
            let t303 = f64x8::splat(3.0) * t302;
            let t304 = t172 * t194;
            let t305 = t85 * t304;
            let t306 = f64x8::splat(3.0) * t305;
            let t307 = t150 * t99;
            let t308 = f64x8::splat(3.0) * t307;
            let t309 = t149 * t84;
            let t310 = t309 * t87;
            let t311 = f64x8::splat(3.0) * t310;
            let t312 = t77 * t163;
            let t313 = t312 * t87;
            let t314 = f64x8::splat(3.0) * t313;
            let t315 = t78 * t195;
            let t316 = f64x8::splat(3.0) * t315;
            let t317 = f64x8::splat(6.0) * t281 - f64x8::splat(12.0) * t283 - t288 - t292 + t294 + t297 + t299 - t303 + t306 + t308 - t311 - t314 + t316;
            let t320 = t158 * t14;
            let t329 = -f64x8::splat(2.0) * t16 * t14 * t74 - f64x8::splat(3.0) * t16 * t158 * t74 - t16 * t320 * t74 - f64x8::splat(3.0) * t10 * t146;
            let t330 = t9 * t329;
            let t331 = t330 * t87;
            let t334 = t183 * t25;
            let t344 = t188 * t30;
            let t354 = -f64x8::splat(6.0) * t27 * t183 * t74 - f64x8::splat(6.0) * t32 * t188 * t74 - f64x8::splat(4.0) * t27 * t25 * t74 - f64x8::splat(2.0) * t27 * t334 * t74 - f64x8::splat(4.0) * t32 * t30 * t74 - f64x8::splat(2.0) * t32 * t344 * t74 - f64x8::splat(6.0) * t22 * t146;
            let t355 = t90 * t354;
            let t356 = t19 * t355;
            let t357 = t247 * t69;
            let t358 = t357 * t6;
            let t359 = f64x8::splat(6.0) * t358;
            let t360 = t248 * t74;
            let t361 = f64x8::splat(6.0) * t360;
            let t362 = t250 * t74;
            let t363 = f64x8::splat(24.0) * t362;
            let t364 = t128 * t146;
            let t365 = f64x8::splat(18.0) * t364;
            let t366 = t130 * t146;
            let t367 = f64x8::splat(36.0) * t366;
            let t369 = f64x8::splat(1.0) / t145 / t1;
            let t370 = t71 * t369;
            let t371 = f64x8::splat(24.0) * t370;
            let t372 = f64x8::splat(3.0) * t146;
            let t375 = -f64x8::splat(6.0) * t4 * t369 - t372;
            let t376 = t375 * t18;
            let t377 = t376 * t35;
            let t378 = t219 * t116;
            let t379 = t378 * t124;
            let t382 = t222 * t244;
            let t385 = t227 * t124;
            let t386 = t385 * t244;
            let t391 = t220 * t110;
            let t397 = t112 * t227 * t228;
            let t400 = f64x8::splat(3.0) * t111 * t382 - f64x8::splat(6.0) * t111 * t397 + f64x8::splat(6.0) * t202 * t223 + f64x8::splat(3.0) * t214 * t223 + f64x8::splat(3.0) * t214 * t391 - f64x8::splat(3.0) * t217 * t379 - f64x8::splat(6.0) * t50 * t386 + t288 + t292 - t294 - t297 - t299 + t303 - t306 - t308 + t311 + t314 - t316;
            let t401 = t103 * t216;
            let t406 = t216 * t110;
            let t407 = t40 * t406;
            let t409 = f64x8::splat(1.0) / t218 / t48;
            let t410 = t409 * t66;
            let t413 = t115 * t115;
            let t414 = f64x8::splat(1.0) / t413;
            let t415 = t228 * t124;
            let t416 = t414 * t415;
            let t421 = t199 * t110;
            let t424 = t103 * t213;
            let t431 = t208 * t45;
            let t440 = -f64x8::splat(3.0) * t47 * t208 * t74 - t47 * t431 * t74 - f64x8::splat(2.0) * t47 * t45 * t74 - f64x8::splat(3.0) * t41 * t146;
            let t441 = t40 * t440;
            let t445 = t233 * t56;
            let t455 = t238 * t61;
            let t465 = -f64x8::splat(6.0) * t58 * t233 * t74 - f64x8::splat(6.0) * t63 * t238 * t74 - f64x8::splat(2.0) * t58 * t445 * t74 - f64x8::splat(2.0) * t63 * t455 * t74 - f64x8::splat(4.0) * t58 * t56 * t74 - f64x8::splat(4.0) * t63 * t61 * t74 - f64x8::splat(6.0) * t53 * t146;
            let t466 = t116 * t465;
            let t470 = -f64x8::splat(6.0) * t37 * t369 - t372;
            let t471 = t470 * t49;
            let t473 = t77 * t166;
            let t474 = t473 * t170;
            let t475 = f64x8::splat(3.0) * t474;
            let t476 = t78 * t179;
            let t477 = f64x8::splat(6.0) * t476;
            let t478 = t166 * t84;
            let t479 = t9 * t478;
            let t481 = f64x8::splat(1.0) / t168 / t17;
            let t482 = t481 * t35;
            let t483 = t479 * t482;
            let t484 = f64x8::splat(2.0) * t483;
            let t485 = t89 * t89;
            let t486 = f64x8::splat(1.0) / t485;
            let t487 = t178 * t98;
            let t488 = t486 * t487;
            let t489 = t19 * t488;
            let t490 = f64x8::splat(6.0) * t489;
            let t491 = -f64x8::splat(6.0) * t104 * t229 + f64x8::splat(3.0) * t104 * t245 - f64x8::splat(3.0) * t421 * t113 - f64x8::splat(3.0) * t424 * t113 - t441 * t113 + f64x8::splat(3.0) * t200 * t125 + f64x8::splat(3.0) * t401 * t220 - f64x8::splat(2.0) * t407 * t410 + f64x8::splat(6.0) * t50 * t416 + t50 * t466 - t471 * t66 + t331 - t356 + t377 - t475 + t477 + t484 - t490;
            let t492 = t400 + t491;
            let t493 = t492 * t70;
            let t494 = t493 * t6;
            let t495 = -t331 + t356 + t359 - t361 - t363 + t365 + t367 - t371 - t377 + t494 + t475 - t477 - t484 + t490;
            let tv3rho30 = t267 + t268 - t269 + t270 - t271 - t272 + t273 + t274 - t275 - t276 + t277 - t278 + t279 + t280 + t1 * (t317 + t495);
            acc_v3rho3_0 = tv3rho30;
            let t500 = -f64x8::splat(2.0) * t281 + f64x8::splat(4.0) * t283 - t288 - t292 + t294 + t297 + t299 - t303 + t306 + t308 - t311 - t314 + t316;
            let t501 = f64x8::splat(2.0) * t358;
            let t502 = f64x8::splat(8.0) * t362;
            let t503 = f64x8::splat(12.0) * t366;
            let t504 = -t331 + t356 + t501 - t361 - t502 + t365 + t503 - t371 - t377 + t494 + t475 - t477 - t484 + t490;
            let tv3rho31 = t267 + t268 - t269 + t270 - t271 - t272 + t273 + t252 - t275 - t258 + t277 - t278 + t279 - t256 + t1 * (t500 + t504);
            acc_v3rho3_1 = tv3rho31;
            let t507 = -t331 + t356 - t501 - t361 + t502 + t365 - t503 - t371 - t377 + t494 + t475 - t477 - t484 + t490;
            let tv3rho32 = t267 + t268 - t269 + t270 - t271 - t272 + t273 - t252 - t275 + t258 + t277 - t278 + t279 - t256 + t1 * (t500 + t507);
            acc_v3rho3_2 = tv3rho32;
            let t510 = -t331 + t356 - t359 - t361 + t363 + t365 - t367 - t371 - t377 + t494 + t475 - t477 - t484 + t490;
            let tv3rho33 = t267 + t268 - t269 + t270 - t271 - t272 + t273 - t274 - t275 + t276 + t277 - t278 + t279 + t280 + t1 * (t317 + t510);
            acc_v3rho3_3 = tv3rho33;
            let t513 = t68 * t146;
            let t514 = f64x8::splat(72.0) * t513;
            let t515 = t127 * t74;
            let t516 = f64x8::splat(48.0) * t515;
            let t517 = t247 * t6;
            let t518 = f64x8::splat(12.0) * t517;
            let t519 = f64x8::splat(12.0) * t369;
            let t521 = f64x8::splat(1.0) / t145 / t5;
            let t526 = (f64x8::splat(30.0) * t4 * t521 + t519) * t18 * t35;
            let t550 = f64x8::splat(8.0) * t479 * t481 * t90 * t98;
            let t553 = f64x8::splat(6.0) * t167 * t300 * t194;
            let t556 = f64x8::splat(4.0) * t85 * t172 * t354;
            let t560 = f64x8::splat(36.0) * t19 * t486 * t178 * t194;
            let t563 = f64x8::splat(8.0) * t19 * t285 * t354;
            let t565 = f64x8::splat(12.0) * t309 * t173;
            let t567 = f64x8::splat(12.0) * t312 * t173;
            let t569 = f64x8::splat(12.0) * t152 * t304;
            let t571 = f64x8::splat(24.0) * t78 * t286;
            let t573 = f64x8::splat(4.0) * t330 * t295;
            let t574 = -(f64x8::splat(30.0) * t37 * t521 + t519) * t49 * t66 + t526 - f64x8::splat(24.0) * t202 * t397 - f64x8::splat(12.0) * t214 * t397 + f64x8::splat(12.0) * t202 * t220 * t213 - f64x8::splat(12.0) * t401 * t379 + f64x8::splat(12.0) * t421 * t223 + f64x8::splat(12.0) * t424 * t223 + f64x8::splat(12.0) * t202 * t382 - t550 + t553 - t556 - t560 + t563 - t565 - t567 - t569 + t571 - t573;
            let t576 = f64x8::splat(4.0) * t330 * t173;
            let t578 = f64x8::splat(6.0) * t164 * t304;
            let t582 = f64x8::splat(12.0) * t167 * t169 * t177 * t178;
            let t586 = f64x8::splat(24.0) * t85 * t86 * t486 * t487;
            let t589 = f64x8::splat(12.0) * t164 * t482 * t166;
            let t591 = f64x8::splat(24.0) * t152 * t290;
            let t593 = f64x8::splat(12.0) * t164 * t290;
            let t596 = f64x8::splat(12.0) * t152 * t170 * t163;
            let t598 = f64x8::splat(12.0) * t473 * t301;
            let t600 = f64x8::splat(4.0) * t78 * t355;
            let t603 = t158 * t158;
            let t617 = t9 * (f64x8::splat(6.0) * t16 * t14 * t146 + f64x8::splat(11.0) * t16 * t158 * t146 + f64x8::splat(6.0) * t16 * t320 * t146 + t16 * t603 * t146 + f64x8::splat(12.0) * t10 * t369) * t87;
            let t620 = t183 * t183;
            let t633 = t188 * t188;
            let t648 = t19 * t90 * (f64x8::splat(22.0) * t27 * t183 * t146 + f64x8::splat(22.0) * t32 * t188 * t146 + f64x8::splat(12.0) * t27 * t25 * t146 + f64x8::splat(12.0) * t27 * t334 * t146 + f64x8::splat(2.0) * t27 * t620 * t146 + f64x8::splat(12.0) * t32 * t30 * t146 + f64x8::splat(12.0) * t32 * t344 * t146 + f64x8::splat(2.0) * t32 * t633 * t146 + f64x8::splat(24.0) * t22 * t369);
            let t651 = t178 * t178;
            let t654 = f64x8::splat(24.0) * t19 / t485 / t34 * t651;
            let t655 = t194 * t194;
            let t658 = f64x8::splat(6.0) * t19 * t177 * t655;
            let t661 = f64x8::splat(6.0) * t149 * t166 * t170;
            let t663 = f64x8::splat(12.0) * t150 * t179;
            let t665 = f64x8::splat(4.0) * t376 * t99;
            let t667 = f64x8::splat(6.0) * t150 * t195;
            let t670 = f64x8::splat(4.0) * t375 * t84 * t87;
            let t671 = -t576 - t578 - t582 - t586 + t589 + t591 + t593 - t596 + t598 - t600 + t617 - t648 + t654 + t658 - t661 + t663 - t665 - t667 + t670;
            let t675 = f64x8::splat(6.0) * t149 * t163 * t87;
            let t678 = f64x8::splat(4.0) * t77 * t329 * t87;
            let t680 = f64x8::splat(24.0) * t78 * t488;
            let t681 = t163 * t163;
            let t684 = f64x8::splat(3.0) * t9 * t681 * t170;
            let t685 = t166 * t166;
            let t687 = t168 * t168;
            let t691 = f64x8::splat(6.0) * t9 * t685 / t687 * t35;
            let t694 = f64x8::splat(8.0) * t77 * t478 * t482;
            let t722 = f64x8::splat(24.0) * t85 * t86 * t286;
            let t727 = f64x8::splat(12.0) * t164 * t169 * t90 * t84 * t98;
            let t738 = t675 + t678 - t680 - t684 - t691 + t694 - f64x8::splat(12.0) * t214 * t410 * t216 + f64x8::splat(12.0) * t217 * t219 * t227 * t228 + f64x8::splat(24.0) * t111 * t112 * t414 * t415 + f64x8::splat(36.0) * t50 * t414 * t228 * t244 - f64x8::splat(8.0) * t50 * t385 * t465 + f64x8::splat(4.0) * t111 * t222 * t465 - f64x8::splat(24.0) * t104 * t386 + f64x8::splat(4.0) * t441 * t391 + t722 + t727 - f64x8::splat(24.0) * t111 * t112 * t386 - f64x8::splat(12.0) * t214 * t219 * t116 * t110 * t124 + f64x8::splat(4.0) * t441 * t223;
            let t775 = t208 * t208;
            let t792 = t233 * t233;
            let t805 = t238 * t238;
            let t821 = t244 * t244;
            let t825 = t213 * t213;
            let t829 = t216 * t216;
            let t831 = t218 * t218;
            let t838 = t228 * t228;
            let t842 = f64x8::splat(6.0) * t214 * t382 + f64x8::splat(8.0) * t407 * t409 * t116 * t124 - f64x8::splat(6.0) * t217 * t378 * t244 - f64x8::splat(8.0) * t103 * t406 * t410 + f64x8::splat(24.0) * t104 * t416 + f64x8::splat(6.0) * t199 * t216 * t220 - f64x8::splat(12.0) * t200 * t229 + f64x8::splat(4.0) * t471 * t125 + f64x8::splat(6.0) * t200 * t245 - f64x8::splat(4.0) * t470 * t110 * t113 - f64x8::splat(6.0) * t199 * t213 * t113 - f64x8::splat(4.0) * t103 * t440 * t113 + f64x8::splat(4.0) * t104 * t466 - t40 * (f64x8::splat(11.0) * t47 * t208 * t146 + f64x8::splat(6.0) * t47 * t431 * t146 + f64x8::splat(6.0) * t47 * t45 * t146 + t47 * t775 * t146 + f64x8::splat(12.0) * t41 * t369) * t113 + t50 * t116 * (f64x8::splat(22.0) * t58 * t233 * t146 + f64x8::splat(22.0) * t63 * t238 * t146 + f64x8::splat(12.0) * t58 * t445 * t146 + f64x8::splat(12.0) * t63 * t455 * t146 + f64x8::splat(12.0) * t58 * t56 * t146 + f64x8::splat(2.0) * t58 * t792 * t146 + f64x8::splat(12.0) * t63 * t61 * t146 + f64x8::splat(2.0) * t63 * t805 * t146 + f64x8::splat(24.0) * t53 * t369) - f64x8::splat(6.0) * t50 * t227 * t821 + f64x8::splat(3.0) * t40 * t825 * t220 + f64x8::splat(6.0) * t40 * t829 / t831 * t66 - f64x8::splat(24.0) * t50 / t413 / t65 * t838;
            let t846 = (t574 + t671 + t738 + t842) * t70 * t6;
            let t848 = f64x8::splat(8.0) * t493 * t74;
            let t850 = t492 * t69 * t6;
            let t851 = f64x8::splat(8.0) * t850;
            let t853 = f64x8::splat(96.0) * t128 * t369;
            let t854 = t130 * t369;
            let t855 = f64x8::splat(192.0) * t854;
            let t857 = f64x8::splat(120.0) * t71 * t521;
            let t859 = f64x8::splat(36.0) * t248 * t146;
            let t860 = t250 * t146;
            let t861 = f64x8::splat(144.0) * t860;
            let t862 = t514 - t516 + t518 - t526 + t846 - t848 + t851 - t853 - t855 + t857 + t859 + t861;
            let t863 = t550 - t553 + t556 + t560 - t563 + t565 + t567 + t569 - t571 + t573 + t576 + t578 + t582;
            let t865 = t586 - t589 - t591 - t593 + t596 - t598 + t600 - t617 + t648 - t654 - t658 + t661;
            let t866 = t357 * t74;
            let t867 = f64x8::splat(48.0) * t866;
            let t868 = -t663 + t665 + t667 - t670 - t675 - t678 + t680 + t684 + t691 - t694 - t867 - t722 - t727;
            let t872 = f64x8::splat(24.0) * t281;
            let t873 = f64x8::splat(48.0) * t283;
            let t874 = f64x8::splat(24.0) * t287;
            let t875 = f64x8::splat(24.0) * t291;
            let t876 = f64x8::splat(24.0) * t293;
            let t877 = f64x8::splat(12.0) * t296;
            let t878 = f64x8::splat(12.0) * t298;
            let t879 = f64x8::splat(12.0) * t302;
            let t880 = f64x8::splat(12.0) * t305;
            let t881 = f64x8::splat(12.0) * t307;
            let t882 = f64x8::splat(12.0) * t310;
            let t883 = f64x8::splat(12.0) * t313;
            let t884 = f64x8::splat(12.0) * t315;
            let t885 = t1 * (t862 + t863 + t865 + t868) + t872 - t873 - t874 - t875 + t876 + t877 + t878 - t879 + t880 + t881 - t882 - t883 + t884;
            let t886 = f64x8::splat(4.0) * t331;
            let t887 = f64x8::splat(4.0) * t356;
            let t888 = f64x8::splat(24.0) * t358;
            let t889 = f64x8::splat(24.0) * t360;
            let t890 = f64x8::splat(96.0) * t362;
            let t891 = f64x8::splat(72.0) * t364;
            let t892 = f64x8::splat(144.0) * t366;
            let t893 = f64x8::splat(96.0) * t370;
            let t894 = f64x8::splat(4.0) * t377;
            let t895 = f64x8::splat(4.0) * t494;
            let t896 = f64x8::splat(12.0) * t474;
            let t897 = f64x8::splat(24.0) * t476;
            let t898 = f64x8::splat(8.0) * t483;
            let t899 = f64x8::splat(24.0) * t489;
            let t900 = -t886 + t887 + t888 - t889 - t890 + t891 + t892 - t893 - t894 + t895 + t896 - t897 - t898 + t899;
            let tv4rho40 = t885 + t900;
            acc_v4rho4_0 = tv4rho40;
            let t901 = f64x8::splat(4.0) * t850;
            let t902 = f64x8::splat(96.0) * t854;
            let t903 = f64x8::splat(72.0) * t860;
            let t904 = -t526 + t846 - t848 + t901 - t853 - t902 + t857 + t859 + t903 + t550 - t553;
            let t905 = t556 + t560 - t563 + t565 + t567 + t569 - t571 + t573 + t576 + t578 + t582 + t586;
            let t907 = -t589 - t591 - t593 + t596 - t598 + t600 - t617 + t648 - t654 - t658 + t661 - t663;
            let t908 = f64x8::splat(24.0) * t866;
            let t909 = t665 + t667 - t670 - t675 - t678 + t680 + t684 + t691 - t694 - t908 - t722 - t727;
            let t913 = t1 * (t904 + t905 + t907 + t909) - t874 - t875 + t876 + t877 + t878 - t879 + t880 + t881 - t882 - t883 + t884 - t886;
            let t914 = f64x8::splat(12.0) * t358;
            let t915 = f64x8::splat(48.0) * t362;
            let t916 = f64x8::splat(72.0) * t366;
            let t917 = t887 + t914 - t889 - t915 + t891 + t916 - t893 - t894 + t895 + t896 - t897 - t898 + t899;
            let tv4rho41 = t913 + t917;
            acc_v4rho4_1 = tv4rho41;
            let t923 = -f64x8::splat(24.0) * t513 + f64x8::splat(16.0) * t515 - f64x8::splat(4.0) * t517 - t526 + t846 - t848 - t853 + t857 + t859 + t550 - t553;
            let t925 = -t589 - t591 - t593 + t596 - t598 + t600 - t617 + t648 - t654 - t658 + t661;
            let t926 = -t663 + t665 + t667 - t670 - t675 - t678 + t680 + t684 + t691 - t694 - t722 - t727;
            let t930 = -f64x8::splat(8.0) * t281 + f64x8::splat(16.0) * t283 + t1 * (t923 + t905 + t925 + t926) - t874 - t875 + t876 + t877 + t878 - t879 + t880 + t881 - t882;
            let t931 = -t883 + t884 - t886 + t887 - t889 + t891 - t893 - t894 + t895 + t896 - t897 - t898 + t899;
            let tv4rho42 = t930 + t931;
            acc_v4rho4_2 = tv4rho42;
            let t932 = -t526 + t846 - t848 - t901 - t853 + t902 + t857 + t859 - t903 + t550 - t553;
            let t934 = t665 + t667 - t670 - t675 - t678 + t680 + t684 + t691 - t694 + t908 - t722 - t727;
            let t938 = t1 * (t932 + t905 + t907 + t934) - t874 - t875 + t876 + t877 + t878 - t879 + t880 + t881 - t882 - t883 + t884 - t886;
            let t939 = t887 - t914 - t889 + t915 + t891 - t916 - t893 - t894 + t895 + t896 - t897 - t898 + t899;
            let tv4rho43 = t938 + t939;
            acc_v4rho4_3 = tv4rho43;
            let t940 = t514 - t516 + t518 - t526 + t846 - t848 - t851 - t853 + t855 + t857 + t859 - t861;
            let t942 = -t663 + t665 + t667 - t670 - t675 - t678 + t680 + t684 + t691 - t694 + t867 - t722 - t727;
            let t946 = t1 * (t940 + t863 + t865 + t942) + t872 - t873 - t874 - t875 + t876 + t877 + t878 - t879 + t880 + t881 - t882 - t883 + t884;
            let t947 = -t886 + t887 - t888 - t889 + t890 + t891 - t892 - t893 - t894 + t895 + t896 - t897 - t898 + t899;
            let tv4rho44 = t946 + t947;
            acc_v4rho4_4 = tv4rho44;
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
        store_strided(v4rho4, ip, m, 5, 0, acc_v4rho4_0);
        store_strided(v4rho4, ip, m, 5, 1, acc_v4rho4_1);
        store_strided(v4rho4, ip, m, 5, 2, acc_v4rho4_2);
        store_strided(v4rho4, ip, m, 5, 3, acc_v4rho4_3);
        store_strided(v4rho4, ip, m, 5, 4, acc_v4rho4_4);
        ip += 8;
    }
}
