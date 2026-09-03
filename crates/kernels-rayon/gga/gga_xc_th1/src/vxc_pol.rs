//! GGA_XC_TH1 vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_xc_th1.c`
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
pub fn gga_xc_th1_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_omega_0: f64,
    param_omega_1: f64,
    param_omega_2: f64,
    param_omega_3: f64,
    param_omega_4: f64,
    param_omega_5: f64,
    param_omega_6: f64,
    param_omega_7: f64,
    param_omega_8: f64,
    param_omega_9: f64,
    param_omega_10: f64,
    param_omega_11: f64,
    param_omega_12: f64,
    param_omega_13: f64,
    param_omega_14: f64,
    param_omega_15: f64,
    param_omega_20: f64,
    param_omega_16: f64,
    param_omega_17: f64,
    param_omega_18: f64,
    param_omega_19: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_omega_0 = f64x8::splat(param_omega_0);
    let param_omega_1 = f64x8::splat(param_omega_1);
    let param_omega_2 = f64x8::splat(param_omega_2);
    let param_omega_3 = f64x8::splat(param_omega_3);
    let param_omega_4 = f64x8::splat(param_omega_4);
    let param_omega_5 = f64x8::splat(param_omega_5);
    let param_omega_6 = f64x8::splat(param_omega_6);
    let param_omega_7 = f64x8::splat(param_omega_7);
    let param_omega_8 = f64x8::splat(param_omega_8);
    let param_omega_9 = f64x8::splat(param_omega_9);
    let param_omega_10 = f64x8::splat(param_omega_10);
    let param_omega_11 = f64x8::splat(param_omega_11);
    let param_omega_12 = f64x8::splat(param_omega_12);
    let param_omega_13 = f64x8::splat(param_omega_13);
    let param_omega_14 = f64x8::splat(param_omega_14);
    let param_omega_15 = f64x8::splat(param_omega_15);
    let param_omega_20 = f64x8::splat(param_omega_20);
    let param_omega_16 = f64x8::splat(param_omega_16);
    let param_omega_17 = f64x8::splat(param_omega_17);
    let param_omega_18 = f64x8::splat(param_omega_18);
    let param_omega_19 = f64x8::splat(param_omega_19);
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
        let mut acc_zk = V_ZERO;
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        let mut acc_vsigma_0 = V_ZERO;
        let mut acc_vsigma_1 = V_ZERO;
        let mut acc_vsigma_2 = V_ZERO;
        {
            let t1 = param_omega_0;
            let t2 = (simd::pow(v_rho0, f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t3 = t2 * v_rho0;
            let t4 = (simd::pow(v_rho1, f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t5 = t4 * v_rho1;
            let t6 = t3 + t5;
            let t8 = param_omega_1;
            let t9 = (simd::cbrt(v_rho0));
            let t10 = t9 * v_rho0;
            let t11 = (simd::cbrt(v_rho1));
            let t12 = t11 * v_rho1;
            let t13 = t10 + t12;
            let t15 = param_omega_2;
            let t16 = ((v_rho0).sqrt());
            let t17 = t16 * v_rho0;
            let t18 = ((v_rho1).sqrt());
            let t19 = t18 * v_rho1;
            let t20 = t17 + t19;
            let t22 = param_omega_3;
            let t23 = t9 * t9;
            let t24 = t23 * v_rho0;
            let t25 = t11 * t11;
            let t26 = t25 * v_rho1;
            let t27 = t24 + t26;
            let t29 = param_omega_4;
            let t30 = t29 * t13;
            let t31 = ((v_sigma0).sqrt());
            let t32 = f64x8::splat(1.0) / t10;
            let t33 = t31 * t32;
            let t34 = v_rho0 - v_rho1;
            let t35 = v_rho0 + v_rho1;
            let t36 = f64x8::splat(1.0) / t35;
            let t37 = t34 * t36;
            let t38 = f64x8::splat(1.0) + t37;
            let t39 = (t38).simd_le(zeta_threshold);
            let t40 = (simd::cbrt(zeta_threshold));
            let t41 = t40 * zeta_threshold;
            let t42 = (simd::cbrt(t38));
            let t44 = ((t39).select(t41, t42 * t38));
            let t45 = f64x8::splat(M_CBRT2);
            let t46 = t45 * t45;
            let t47 = t44 * t46;
            let t49 = ((v_sigma2).sqrt());
            let t50 = f64x8::splat(1.0) / t12;
            let t51 = t49 * t50;
            let t52 = f64x8::splat(1.0) - t37;
            let t53 = (t52).simd_le(zeta_threshold);
            let t54 = (simd::cbrt(t52));
            let t56 = ((t53).select(t41, t54 * t52));
            let t57 = t56 * t46;
            let t60 = t33 * t47 / f64x8::splat(4.0) + t51 * t57 / f64x8::splat(4.0);
            let t63 = param_omega_5;
            let t64 = t63 * t20;
            let t67 = param_omega_6;
            let t68 = t67 * t27;
            let t71 = param_omega_7;
            let t72 = t2 * t2;
            let t73 = t72 * t72;
            let t74 = t73 * t2;
            let t75 = t74 * v_rho0;
            let t76 = t4 * t4;
            let t77 = t76 * t76;
            let t78 = t77 * t4;
            let t79 = t78 * v_rho1;
            let t80 = t75 + t79;
            let t81 = t71 * t80;
            let t84 = param_omega_8;
            let t85 = t84 * t20;
            let t86 = v_rho0 * v_rho0;
            let t88 = f64x8::splat(1.0) / t23 / t86;
            let t89 = v_sigma0 * t88;
            let t90 = t44 * t44;
            let t91 = t90 * t45;
            let t92 = t89 * t91;
            let t93 = v_rho1 * v_rho1;
            let t95 = f64x8::splat(1.0) / t25 / t93;
            let t96 = v_sigma2 * t95;
            let t97 = t56 * t56;
            let t98 = t97 * t45;
            let t99 = t96 * t98;
            let t101 = t92 / f64x8::splat(8.0) + t99 / f64x8::splat(8.0);
            let t104 = param_omega_9;
            let t105 = t104 * t27;
            let t109 = param_omega_10;
            let t110 = t109 * t80;
            let t113 = param_omega_11;
            let t114 = t86 + t93;
            let t115 = t113 * t114;
            let t118 = param_omega_12;
            let t119 = t118 * t20;
            let t123 = v_sigma0 + f64x8::splat(2.0) * v_sigma1 + v_sigma2;
            let t124 = t35 * t35;
            let t125 = (simd::cbrt(t35));
            let t126 = t125 * t125;
            let t128 = f64x8::splat(1.0) / t126 / t124;
            let t130 = t92 / f64x8::splat(4.0) + t99 / f64x8::splat(4.0) - t123 * t128;
            let t132 = param_omega_13;
            let t133 = t132 * t27;
            let t135 = param_omega_14;
            let t136 = t135 * t80;
            let t138 = param_omega_15;
            let t139 = t138 * t114;
            let t141 = param_omega_16;
            let t142 = t141 * t6;
            let t143 = t34 * t34;
            let t144 = f64x8::splat(1.0) / t124;
            let t145 = t143 * t144;
            let t147 = param_omega_17;
            let t148 = t147 * t13;
            let t150 = param_omega_18;
            let t151 = t150 * t20;
            let t153 = param_omega_19;
            let t154 = t153 * t27;
            let t156 = param_omega_20;
            let t158 = t110 * t101 / f64x8::splat(2.0) + t115 * t101 / f64x8::splat(2.0) + t119 * t130 + t133 * t130 + t136 * t130 + t139 * t130 + t142 * t145 + t148 * t145 + t151 * t145 + t154 * t145 + t156 * t35;
            let tzk0 = (t1 * t6 + t8 * t13 + t15 * t20 + t22 * t27 + t30 * t60 / f64x8::splat(2.0) + t64 * t60 / f64x8::splat(2.0) + t68 * t60 / f64x8::splat(2.0) + t81 * t60 / f64x8::splat(2.0) + t85 * t101 / f64x8::splat(2.0) + t105 * t101 / f64x8::splat(2.0) + t158) * t36;
            acc_zk = tzk0;
            let t168 = t118 * t16;
            let t171 = t86 * v_rho0;
            let t173 = f64x8::splat(1.0) / t23 / t171;
            let t174 = v_sigma0 * t173;
            let t175 = t174 * t91;
            let t177 = t44 * t45;
            let t178 = t34 * t144;
            let t179 = t36 - t178;
            let t182 = ((t39).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t42 * t179));
            let t183 = t177 * t182;
            let t184 = t89 * t183;
            let t186 = t56 * t45;
            let t187 = -t179;
            let t190 = ((t53).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t54 * t187));
            let t191 = t186 * t190;
            let t192 = t96 * t191;
            let t194 = t124 * t35;
            let t196 = f64x8::splat(1.0) / t126 / t194;
            let t198 = f64x8::splat(8.0) / f64x8::splat(3.0) * t123 * t196;
            let t199 = -f64x8::splat(2.0) / f64x8::splat(3.0) * t175 + t184 / f64x8::splat(2.0) + t192 / f64x8::splat(2.0) + t198;
            let t201 = t132 * t23;
            let t205 = t135 * t74;
            let t209 = t138 * v_rho0;
            let t213 = f64x8::splat(1.0) / t9 / t86;
            let t214 = t31 * t213;
            let t217 = t182 * t46;
            let t220 = t190 * t46;
            let t223 = -t214 * t47 / f64x8::splat(3.0) + t33 * t217 / f64x8::splat(4.0) + t51 * t220 / f64x8::splat(4.0);
            let t226 = t29 * t9;
            let t231 = t63 * t16;
            let t236 = t67 * t23;
            let t241 = t71 * t74;
            let t244 = t156 + f64x8::splat(7.0) / f64x8::splat(6.0) * t1 * t2 + f64x8::splat(4.0) / f64x8::splat(3.0) * t8 * t9 + f64x8::splat(3.0) / f64x8::splat(2.0) * t15 * t16 + f64x8::splat(5.0) / f64x8::splat(3.0) * t22 * t23 + f64x8::splat(3.0) / f64x8::splat(2.0) * t168 * t130 + t133 * t199 + f64x8::splat(5.0) / f64x8::splat(3.0) * t201 * t130 + t136 * t199 + f64x8::splat(11.0) / f64x8::splat(6.0) * t205 * t130 + t139 * t199 + f64x8::splat(2.0) * t209 * t130 + t30 * t223 / f64x8::splat(2.0) + f64x8::splat(2.0) / f64x8::splat(3.0) * t226 * t60 + t64 * t223 / f64x8::splat(2.0) + f64x8::splat(3.0) / f64x8::splat(4.0) * t231 * t60 + t68 * t223 / f64x8::splat(2.0) + f64x8::splat(5.0) / f64x8::splat(6.0) * t236 * t60 + t81 * t223 / f64x8::splat(2.0) + f64x8::splat(11.0) / f64x8::splat(12.0) * t241 * t60;
            let t248 = -t175 / f64x8::splat(3.0) + t184 / f64x8::splat(4.0) + t192 / f64x8::splat(4.0);
            let t251 = t84 * t16;
            let t256 = t104 * t23;
            let t261 = t109 * t74;
            let t266 = t113 * v_rho0;
            let t269 = f64x8::splat(1.0) / t194;
            let t270 = t143 * t269;
            let t272 = f64x8::splat(2.0) * t154 * t270;
            let t275 = f64x8::splat(2.0) * t142 * t178;
            let t277 = f64x8::splat(2.0) * t142 * t270;
            let t279 = f64x8::splat(2.0) * t148 * t178;
            let t281 = f64x8::splat(2.0) * t148 * t270;
            let t283 = f64x8::splat(2.0) * t151 * t178;
            let t285 = f64x8::splat(2.0) * t151 * t270;
            let t287 = f64x8::splat(2.0) * t154 * t178;
            let t288 = t141 * t2;
            let t291 = t147 * t9;
            let t294 = t150 * t16;
            let t297 = t153 * t23;
            let t300 = t275 - t277 + t279 - t281 + t283 - t285 + t287 + f64x8::splat(7.0) / f64x8::splat(6.0) * t288 * t145 + f64x8::splat(4.0) / f64x8::splat(3.0) * t291 * t145 + f64x8::splat(3.0) / f64x8::splat(2.0) * t294 * t145 + f64x8::splat(5.0) / f64x8::splat(3.0) * t297 * t145;
            let tvrho0 = t244 + t85 * t248 / f64x8::splat(2.0) + f64x8::splat(3.0) / f64x8::splat(4.0) * t251 * t101 + t105 * t248 / f64x8::splat(2.0) + f64x8::splat(5.0) / f64x8::splat(6.0) * t256 * t101 + t110 * t248 / f64x8::splat(2.0) + f64x8::splat(11.0) / f64x8::splat(12.0) * t261 * t101 + t115 * t248 / f64x8::splat(2.0) + t266 * t101 + t119 * t199 - t272 + t300;
            acc_vrho_0 = tvrho0;
            let t310 = -t36 - t178;
            let t313 = ((t39).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t42 * t310));
            let t314 = t177 * t313;
            let t315 = t89 * t314;
            let t317 = t93 * v_rho1;
            let t319 = f64x8::splat(1.0) / t25 / t317;
            let t320 = v_sigma2 * t319;
            let t321 = t320 * t98;
            let t323 = -t310;
            let t326 = ((t53).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t54 * t323));
            let t327 = t186 * t326;
            let t328 = t96 * t327;
            let t330 = t315 / f64x8::splat(2.0) - f64x8::splat(2.0) / f64x8::splat(3.0) * t321 + t328 / f64x8::splat(2.0) + t198;
            let t332 = t132 * t25;
            let t336 = t135 * t78;
            let t340 = t138 * v_rho1;
            let t343 = t313 * t46;
            let t347 = f64x8::splat(1.0) / t11 / t93;
            let t348 = t49 * t347;
            let t351 = t326 * t46;
            let t354 = t33 * t343 / f64x8::splat(4.0) - t348 * t57 / f64x8::splat(3.0) + t51 * t351 / f64x8::splat(4.0);
            let t357 = t29 * t11;
            let t362 = t63 * t18;
            let t367 = t67 * t25;
            let t372 = t71 * t78;
            let t378 = t315 / f64x8::splat(4.0) - t321 / f64x8::splat(3.0) + t328 / f64x8::splat(4.0);
            let t381 = t156 + f64x8::splat(7.0) / f64x8::splat(6.0) * t1 * t4 + f64x8::splat(4.0) / f64x8::splat(3.0) * t8 * t11 + f64x8::splat(3.0) / f64x8::splat(2.0) * t15 * t18 + f64x8::splat(5.0) / f64x8::splat(3.0) * t22 * t25 + t133 * t330 + f64x8::splat(5.0) / f64x8::splat(3.0) * t332 * t130 + t136 * t330 + f64x8::splat(11.0) / f64x8::splat(6.0) * t336 * t130 + t139 * t330 + f64x8::splat(2.0) * t340 * t130 + t30 * t354 / f64x8::splat(2.0) + f64x8::splat(2.0) / f64x8::splat(3.0) * t357 * t60 + t64 * t354 / f64x8::splat(2.0) + f64x8::splat(3.0) / f64x8::splat(4.0) * t362 * t60 + t68 * t354 / f64x8::splat(2.0) + f64x8::splat(5.0) / f64x8::splat(6.0) * t367 * t60 + t81 * t354 / f64x8::splat(2.0) + f64x8::splat(11.0) / f64x8::splat(12.0) * t372 * t60 + t85 * t378 / f64x8::splat(2.0);
            let t382 = t84 * t18;
            let t387 = t104 * t25;
            let t392 = t109 * t78;
            let t397 = t113 * v_rho1;
            let t400 = t118 * t18;
            let t404 = t141 * t4;
            let t407 = t147 * t11;
            let t410 = t150 * t18;
            let t413 = t153 * t25;
            let t416 = f64x8::splat(7.0) / f64x8::splat(6.0) * t404 * t145 + f64x8::splat(4.0) / f64x8::splat(3.0) * t407 * t145 + f64x8::splat(3.0) / f64x8::splat(2.0) * t410 * t145 + f64x8::splat(5.0) / f64x8::splat(3.0) * t413 * t145 - t275 - t277 - t279 - t281 - t283 - t285 - t287;
            let tvrho1 = t381 + f64x8::splat(3.0) / f64x8::splat(4.0) * t382 * t101 + t105 * t378 / f64x8::splat(2.0) + f64x8::splat(5.0) / f64x8::splat(6.0) * t387 * t101 + t110 * t378 / f64x8::splat(2.0) + f64x8::splat(11.0) / f64x8::splat(12.0) * t392 * t101 + t115 * t378 / f64x8::splat(2.0) + t397 * t101 + t119 * t330 + f64x8::splat(3.0) / f64x8::splat(2.0) * t400 * t130 - t272 + t416;
            acc_vrho_1 = tvrho1;
            let t418 = f64x8::splat(1.0) / t31;
            let t419 = t30 * t418;
            let t421 = t32 * t44 * t46;
            let t424 = t64 * t418;
            let t427 = t68 * t418;
            let t430 = t81 * t418;
            let t434 = t88 * t90 * t45;
            let t444 = t434 / f64x8::splat(4.0) - t128;
            let tvsigma0 = t419 * t421 / f64x8::splat(16.0) + t424 * t421 / f64x8::splat(16.0) + t427 * t421 / f64x8::splat(16.0) + t430 * t421 / f64x8::splat(16.0) + t85 * t434 / f64x8::splat(16.0) + t105 * t434 / f64x8::splat(16.0) + t110 * t434 / f64x8::splat(16.0) + t115 * t434 / f64x8::splat(16.0) + t119 * t444 + t133 * t444 + t136 * t444 + t139 * t444;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = -f64x8::splat(2.0) * t119 * t128 - f64x8::splat(2.0) * t133 * t128 - f64x8::splat(2.0) * t136 * t128 - f64x8::splat(2.0) * t139 * t128;
            acc_vsigma_1 = tvsigma1;
            let t454 = f64x8::splat(1.0) / t49;
            let t455 = t30 * t454;
            let t457 = t50 * t56 * t46;
            let t460 = t64 * t454;
            let t463 = t68 * t454;
            let t466 = t81 * t454;
            let t470 = t95 * t97 * t45;
            let t480 = t470 / f64x8::splat(4.0) - t128;
            let tvsigma2 = t455 * t457 / f64x8::splat(16.0) + t460 * t457 / f64x8::splat(16.0) + t463 * t457 / f64x8::splat(16.0) + t466 * t457 / f64x8::splat(16.0) + t85 * t470 / f64x8::splat(16.0) + t105 * t470 / f64x8::splat(16.0) + t110 * t470 / f64x8::splat(16.0) + t115 * t470 / f64x8::splat(16.0) + t119 * t480 + t133 * t480 + t136 * t480 + t139 * t480;
            acc_vsigma_2 = tvsigma2;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(vsigma, ip, m, 3, 0, acc_vsigma_0);
        store_strided(vsigma, ip, m, 3, 1, acc_vsigma_1);
        store_strided(vsigma, ip, m, 3, 2, acc_vsigma_2);
        ip += 8;
    }
}
