//! MGGA_X_MVSB vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_mvsb.c`
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
pub fn mgga_x_mvsb_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    param_b: f64,
    param_c1: f64,
    param_e1: f64,
    param_k0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_b = f64x8::splat(param_b);
    let param_c1 = f64x8::splat(param_c1);
    let param_e1 = f64x8::splat(param_e1);
    let param_k0 = f64x8::splat(param_k0);
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
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        let mut acc_vsigma_0 = V_ZERO;
        let mut acc_vsigma_1 = V_ZERO;
        let mut acc_vsigma_2 = V_ZERO;
        let mut acc_vlapl_0 = V_ZERO;
        let mut acc_vlapl_1 = V_ZERO;
        let mut acc_vtau_0 = V_ZERO;
        let mut acc_vtau_1 = V_ZERO;
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
            let t19 = ((t11).select(t12, (t15).select(t16, t17 * t8)));
            let t20 = f64x8::splat(1.0) + t19;
            let t21 = (t20).simd_le(zeta_threshold);
            let t22 = (simd::cbrt(zeta_threshold));
            let t23 = t22 * zeta_threshold;
            let t24 = (simd::cbrt(t20));
            let t26 = ((t21).select(t23, t24 * t20));
            let t27 = t6 * t26;
            let t28 = (simd::cbrt(t7));
            let t29 = (simd::cbrt(v_rho0));
            let t30 = t29 * t29;
            let t32 = f64x8::splat(1.0) / t30 / v_rho0;
            let t33 = v_tau0 * t32;
            let t34 = v_rho0 * v_rho0;
            let t36 = f64x8::splat(1.0) / t30 / t34;
            let t39 = t33 - v_sigma0 * t36 / f64x8::splat(8.0);
            let t40 = f64x8::splat(M_CBRT6);
            let t41 = t40 * t40;
            let t42 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t43 = (simd::cbrt(t42));
            let t44 = t43 * t43;
            let t46 = f64x8::splat(3.0) / f64x8::splat(10.0) * t41 * t44;
            let t47 = t33 - t46;
            let t48 = f64x8::splat(1.0) / t47;
            let t51 = param_k0 * (-t39 * t48 + f64x8::splat(1.0));
            let t52 = t39 * t39;
            let t53 = param_e1 * t52;
            let t54 = t47 * t47;
            let t55 = f64x8::splat(1.0) / t54;
            let t57 = t53 * t55 + f64x8::splat(1.0);
            let t58 = t57 * t57;
            let t59 = t52 * t52;
            let t60 = param_c1 * t59;
            let t61 = t54 * t54;
            let t62 = f64x8::splat(1.0) / t61;
            let t64 = t60 * t62 + t58;
            let t65 = ((t64).sqrt().sqrt());
            let t66 = f64x8::splat(1.0) / t65;
            let t68 = t51 * t66 + f64x8::splat(1.0);
            let t70 = param_b * t41;
            let t72 = f64x8::splat(1.0) / t43 / t42;
            let t73 = v_sigma0 * v_sigma0;
            let t74 = t72 * t73;
            let t75 = t34 * t34;
            let t76 = t75 * v_rho0;
            let t78 = f64x8::splat(1.0) / t29 / t76;
            let t82 = f64x8::splat(1.0) + t70 * t74 * t78 / f64x8::splat(576.0);
            let t83 = (simd::pow(t82, f64x8::splat(1.0) / f64x8::splat(8.0)));
            let t84 = f64x8::splat(1.0) / t83;
            let t85 = t28 * t68 * t84;
            let t88 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t27 * t85));
            let t89 = (v_rho1).simd_le(dens_threshold);
            let t90 = -t17;
            let t92 = ((t15).select(t12, (t11).select(t16, t90 * t8)));
            let t93 = f64x8::splat(1.0) + t92;
            let t94 = (t93).simd_le(zeta_threshold);
            let t95 = (simd::cbrt(t93));
            let t97 = ((t94).select(t23, t95 * t93));
            let t98 = t6 * t97;
            let t99 = (simd::cbrt(v_rho1));
            let t100 = t99 * t99;
            let t102 = f64x8::splat(1.0) / t100 / v_rho1;
            let t103 = v_tau1 * t102;
            let t104 = v_rho1 * v_rho1;
            let t106 = f64x8::splat(1.0) / t100 / t104;
            let t109 = t103 - v_sigma2 * t106 / f64x8::splat(8.0);
            let t110 = t103 - t46;
            let t111 = f64x8::splat(1.0) / t110;
            let t114 = param_k0 * (-t109 * t111 + f64x8::splat(1.0));
            let t115 = t109 * t109;
            let t116 = param_e1 * t115;
            let t117 = t110 * t110;
            let t118 = f64x8::splat(1.0) / t117;
            let t120 = t116 * t118 + f64x8::splat(1.0);
            let t121 = t120 * t120;
            let t122 = t115 * t115;
            let t123 = param_c1 * t122;
            let t124 = t117 * t117;
            let t125 = f64x8::splat(1.0) / t124;
            let t127 = t123 * t125 + t121;
            let t128 = ((t127).sqrt().sqrt());
            let t129 = f64x8::splat(1.0) / t128;
            let t131 = t114 * t129 + f64x8::splat(1.0);
            let t133 = v_sigma2 * v_sigma2;
            let t134 = t72 * t133;
            let t135 = t104 * t104;
            let t136 = t135 * v_rho1;
            let t138 = f64x8::splat(1.0) / t99 / t136;
            let t142 = f64x8::splat(1.0) + t70 * t134 * t138 / f64x8::splat(576.0);
            let t143 = (simd::pow(t142, f64x8::splat(1.0) / f64x8::splat(8.0)));
            let t144 = f64x8::splat(1.0) / t143;
            let t145 = t28 * t131 * t144;
            let t148 = ((t89).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t98 * t145));
            let tzk0 = t88 + t148;
            acc_zk = tzk0;
            let t149 = t7 * t7;
            let t150 = f64x8::splat(1.0) / t149;
            let t151 = t17 * t150;
            let t153 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), t8 - t151)));
            let t156 = ((t21).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t24 * t153));
            let t157 = t6 * t156;
            let t160 = t28 * t28;
            let t161 = f64x8::splat(1.0) / t160;
            let t163 = t161 * t68 * t84;
            let t165 = t27 * t163 / f64x8::splat(8.0);
            let t166 = v_tau0 * t36;
            let t168 = t34 * v_rho0;
            let t170 = f64x8::splat(1.0) / t30 / t168;
            let t173 = -f64x8::splat(5.0) / f64x8::splat(3.0) * t166 + v_sigma0 * t170 / f64x8::splat(3.0);
            let t175 = t39 * t55;
            let t179 = param_k0 * (-t173 * t48 - f64x8::splat(5.0) / f64x8::splat(3.0) * t175 * t166);
            let t182 = f64x8::splat(1.0) / t65 / t64;
            let t183 = param_e1 * t39;
            let t184 = t55 * t173;
            let t187 = t54 * t47;
            let t188 = f64x8::splat(1.0) / t187;
            let t189 = t188 * v_tau0;
            let t190 = t189 * t36;
            let t193 = f64x8::splat(2.0) * t183 * t184 + f64x8::splat(10.0) / f64x8::splat(3.0) * t53 * t190;
            let t197 = param_c1 * t52 * t39;
            let t198 = t62 * t173;
            let t202 = f64x8::splat(1.0) / t61 / t47;
            let t203 = t202 * v_tau0;
            let t207 = f64x8::splat(2.0) * t57 * t193 + f64x8::splat(4.0) * t197 * t198 + f64x8::splat(20.0) / f64x8::splat(3.0) * t60 * t203 * t36;
            let t208 = t182 * t207;
            let t211 = t179 * t66 - t51 * t208 / f64x8::splat(4.0);
            let t213 = t28 * t211 * t84;
            let t216 = t26 * t28;
            let t217 = t216 * t68;
            let t218 = t6 * t217;
            let t221 = f64x8::splat(1.0) / t83 / t82 * param_b;
            let t222 = t221 * t41;
            let t223 = t75 * t34;
            let t225 = f64x8::splat(1.0) / t29 / t223;
            let t227 = t222 * t74 * t225;
            let t231 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t157 * t85 - t165 - f64x8::splat(3.0) / f64x8::splat(8.0) * t27 * t213 - t218 * t227 / f64x8::splat(2304.0)));
            let t232 = t90 * t150;
            let t234 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), -t8 - t232)));
            let t237 = ((t94).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t95 * t234));
            let t238 = t6 * t237;
            let t242 = t161 * t131 * t144;
            let t244 = t98 * t242 / f64x8::splat(8.0);
            let t246 = ((t89).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t238 * t145 - t244));
            let tvrho0 = t88 + t148 + t7 * (t231 + t246);
            acc_vrho_0 = tvrho0;
            let t250 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), -t8 - t151)));
            let t253 = ((t21).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t24 * t250));
            let t254 = t6 * t253;
            let t258 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t254 * t85 - t165));
            let t260 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), t8 - t232)));
            let t263 = ((t94).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t95 * t260));
            let t264 = t6 * t263;
            let t267 = v_tau1 * t106;
            let t269 = t104 * v_rho1;
            let t271 = f64x8::splat(1.0) / t100 / t269;
            let t274 = -f64x8::splat(5.0) / f64x8::splat(3.0) * t267 + v_sigma2 * t271 / f64x8::splat(3.0);
            let t276 = t109 * t118;
            let t280 = param_k0 * (-t274 * t111 - f64x8::splat(5.0) / f64x8::splat(3.0) * t276 * t267);
            let t283 = f64x8::splat(1.0) / t128 / t127;
            let t284 = param_e1 * t109;
            let t285 = t118 * t274;
            let t288 = t117 * t110;
            let t289 = f64x8::splat(1.0) / t288;
            let t290 = t289 * v_tau1;
            let t291 = t290 * t106;
            let t294 = f64x8::splat(2.0) * t284 * t285 + f64x8::splat(10.0) / f64x8::splat(3.0) * t116 * t291;
            let t298 = param_c1 * t115 * t109;
            let t299 = t125 * t274;
            let t303 = f64x8::splat(1.0) / t124 / t110;
            let t304 = t303 * v_tau1;
            let t308 = f64x8::splat(2.0) * t120 * t294 + f64x8::splat(4.0) * t298 * t299 + f64x8::splat(20.0) / f64x8::splat(3.0) * t123 * t304 * t106;
            let t309 = t283 * t308;
            let t312 = t280 * t129 - t114 * t309 / f64x8::splat(4.0);
            let t314 = t28 * t312 * t144;
            let t317 = t97 * t28;
            let t318 = t317 * t131;
            let t319 = t6 * t318;
            let t322 = f64x8::splat(1.0) / t143 / t142 * param_b;
            let t323 = t322 * t41;
            let t324 = t135 * t104;
            let t326 = f64x8::splat(1.0) / t99 / t324;
            let t328 = t323 * t134 * t326;
            let t332 = ((t89).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t264 * t145 - t244 - f64x8::splat(3.0) / f64x8::splat(8.0) * t98 * t314 - t319 * t328 / f64x8::splat(2304.0)));
            let tvrho1 = t88 + t148 + t7 * (t258 + t332);
            acc_vrho_1 = tvrho1;
            let t335 = param_k0 * t36;
            let t336 = t48 * t66;
            let t339 = t57 * param_e1;
            let t340 = t175 * t36;
            let t342 = t62 * t36;
            let t343 = t197 * t342;
            let t345 = -t339 * t340 / f64x8::splat(2.0) - t343 / f64x8::splat(2.0);
            let t346 = t182 * t345;
            let t349 = t335 * t336 / f64x8::splat(8.0) - t51 * t346 / f64x8::splat(4.0);
            let t351 = t28 * t349 * t84;
            let t354 = t72 * v_sigma0;
            let t356 = t222 * t354 * t78;
            let t360 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t27 * t351 + t218 * t356 / f64x8::splat(6144.0)));
            let tvsigma0 = t7 * t360;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t361 = param_k0 * t106;
            let t362 = t111 * t129;
            let t365 = t120 * param_e1;
            let t366 = t276 * t106;
            let t368 = t125 * t106;
            let t369 = t298 * t368;
            let t371 = -t365 * t366 / f64x8::splat(2.0) - t369 / f64x8::splat(2.0);
            let t372 = t283 * t371;
            let t375 = t361 * t362 / f64x8::splat(8.0) - t114 * t372 / f64x8::splat(4.0);
            let t377 = t28 * t375 * t144;
            let t380 = t72 * v_sigma2;
            let t382 = t323 * t380 * t138;
            let t386 = ((t89).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t98 * t377 + t319 * t382 / f64x8::splat(6144.0)));
            let tvsigma2 = t7 * t386;
            acc_vsigma_2 = tvsigma2;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl_0 = tvlapl0;
            let tvlapl1 = f64x8::splat(0.0);
            acc_vlapl_1 = tvlapl1;
            let t390 = param_k0 * (t175 * t32 - t32 * t48);
            let t392 = t55 * t32;
            let t394 = t188 * t32;
            let t397 = f64x8::splat(2.0) * t183 * t392 - f64x8::splat(2.0) * t53 * t394;
            let t400 = t62 * t32;
            let t403 = t202 * t32;
            let t406 = f64x8::splat(4.0) * t197 * t400 + f64x8::splat(2.0) * t57 * t397 - f64x8::splat(4.0) * t60 * t403;
            let t407 = t182 * t406;
            let t410 = t390 * t66 - t51 * t407 / f64x8::splat(4.0);
            let t412 = t28 * t410 * t84;
            let t415 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t27 * t412));
            let tvtau0 = t7 * t415;
            acc_vtau_0 = tvtau0;
            let t419 = param_k0 * (-t102 * t111 + t276 * t102);
            let t421 = t118 * t102;
            let t423 = t289 * t102;
            let t426 = -f64x8::splat(2.0) * t116 * t423 + f64x8::splat(2.0) * t284 * t421;
            let t429 = t125 * t102;
            let t432 = t303 * t102;
            let t435 = f64x8::splat(2.0) * t120 * t426 - f64x8::splat(4.0) * t123 * t432 + f64x8::splat(4.0) * t298 * t429;
            let t436 = t283 * t435;
            let t439 = t419 * t129 - t114 * t436 / f64x8::splat(4.0);
            let t441 = t28 * t439 * t144;
            let t444 = ((t89).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t98 * t441));
            let tvtau1 = t7 * t444;
            acc_vtau_1 = tvtau1;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(vsigma, ip, m, 3, 0, acc_vsigma_0);
        store_strided(vsigma, ip, m, 3, 1, acc_vsigma_1);
        store_strided(vsigma, ip, m, 3, 2, acc_vsigma_2);
        store_strided(vlapl, ip, m, 2, 0, acc_vlapl_0);
        store_strided(vlapl, ip, m, 2, 1, acc_vlapl_1);
        store_strided(vtau, ip, m, 2, 0, acc_vtau_0);
        store_strided(vtau, ip, m, 2, 1, acc_vtau_1);
        ip += 8;
    }
}
