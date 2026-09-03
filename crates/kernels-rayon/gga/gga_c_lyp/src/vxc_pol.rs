//! GGA_C_LYP vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_lyp.c`
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
pub fn gga_c_lyp_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_a: f64,
    param_b: f64,
    param_c: f64,
    param_d: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_a = f64x8::splat(param_a);
    let param_b = f64x8::splat(param_b);
    let param_c = f64x8::splat(param_c);
    let param_d = f64x8::splat(param_d);
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
            let t1 = v_rho0 - v_rho1;
            let t2 = t1 * t1;
            let t3 = v_rho0 + v_rho1;
            let t4 = t3 * t3;
            let t5 = f64x8::splat(1.0) / t4;
            let t7 = -t2 * t5 + f64x8::splat(1.0);
            let t8 = (simd::cbrt(t3));
            let t9 = f64x8::splat(1.0) / t8;
            let t11 = param_d * t9 + f64x8::splat(1.0);
            let t12 = f64x8::splat(1.0) / t11;
            let t15 = (simd::exp(-param_c * t9));
            let t16 = param_b * t15;
            let t18 = v_sigma0 + f64x8::splat(2.0) * v_sigma1 + v_sigma2;
            let t19 = t8 * t8;
            let t21 = f64x8::splat(1.0) / t19 / t4;
            let t22 = t18 * t21;
            let t24 = param_d * t12 + param_c;
            let t25 = t24 * t9;
            let t27 = f64x8::splat(47.0) - f64x8::splat(7.0) * t25;
            let t30 = t7 * t27 / f64x8::splat(72.0) - f64x8::splat(2.0) / f64x8::splat(3.0);
            let t32 = f64x8::splat(M_CBRT3);
            let t33 = t32 * t32;
            let t34 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t35 = (simd::cbrt(t34));
            let t36 = t35 * t35;
            let t37 = t33 * t36;
            let t38 = f64x8::splat(1.0) / t3;
            let t39 = t1 * t38;
            let t40 = f64x8::splat(1.0) + t39;
            let t41 = (t40).simd_le(zeta_threshold);
            let t42 = zeta_threshold * zeta_threshold;
            let t43 = (simd::cbrt(zeta_threshold));
            let t44 = t43 * t43;
            let t45 = t44 * t42;
            let t46 = t40 * t40;
            let t47 = (simd::cbrt(t40));
            let t48 = t47 * t47;
            let t49 = t48 * t46;
            let t50 = ((t41).select(t45, t49));
            let t51 = f64x8::splat(1.0) - t39;
            let t52 = (t51).simd_le(zeta_threshold);
            let t53 = t51 * t51;
            let t54 = (simd::cbrt(t51));
            let t55 = t54 * t54;
            let t56 = t55 * t53;
            let t57 = ((t52).select(t45, t56));
            let t58 = t50 + t57;
            let t62 = f64x8::splat(M_CBRT2);
            let t63 = t62 * t7;
            let t65 = f64x8::splat(5.0) / f64x8::splat(2.0) - t25 / f64x8::splat(18.0);
            let t66 = v_rho0 * v_rho0;
            let t67 = (simd::cbrt(v_rho0));
            let t68 = t67 * t67;
            let t70 = f64x8::splat(1.0) / t68 / t66;
            let t71 = v_sigma0 * t70;
            let t72 = t71 * t50;
            let t73 = v_rho1 * v_rho1;
            let t74 = (simd::cbrt(v_rho1));
            let t75 = t74 * t74;
            let t77 = f64x8::splat(1.0) / t75 / t73;
            let t78 = v_sigma2 * t77;
            let t79 = t78 * t57;
            let t80 = t72 + t79;
            let t81 = t65 * t80;
            let t84 = t25 - f64x8::splat(11.0);
            let t86 = t44 * t42 * zeta_threshold;
            let t89 = ((t41).select(t86, t48 * t46 * t40));
            let t93 = ((t52).select(t86, t55 * t53 * t51));
            let t95 = t71 * t89 + t78 * t93;
            let t96 = t84 * t95;
            let t101 = ((t41).select(t42, t46));
            let t102 = t101 * v_sigma2;
            let t103 = t77 * t57;
            let t106 = ((t52).select(t42, t53));
            let t107 = t106 * v_sigma0;
            let t108 = t70 * t50;
            let t114 = -t22 * t30 - f64x8::splat(3.0) / f64x8::splat(20.0) * t37 * t7 * t58 + t63 * t81 / f64x8::splat(32.0) + t63 * t96 / f64x8::splat(576.0) - t62 * (f64x8::splat(2.0) / f64x8::splat(3.0) * t72 + f64x8::splat(2.0) / f64x8::splat(3.0) * t79 - t102 * t103 / f64x8::splat(4.0) - t107 * t108 / f64x8::splat(4.0)) / f64x8::splat(8.0);
            let tzk0 = param_a * (t16 * t12 * t114 - t7 * t12);
            acc_zk = tzk0;
            let t118 = t3 * param_a;
            let t119 = t1 * t5;
            let t120 = t4 * t3;
            let t121 = f64x8::splat(1.0) / t120;
            let t122 = t2 * t121;
            let t124 = -f64x8::splat(2.0) * t119 + f64x8::splat(2.0) * t122;
            let t126 = t11 * t11;
            let t127 = f64x8::splat(1.0) / t126;
            let t128 = t7 * t127;
            let t130 = f64x8::splat(1.0) / t8 / t3;
            let t131 = param_d * t130;
            let t133 = t128 * t131 / f64x8::splat(3.0);
            let t134 = param_b * param_c;
            let t135 = t134 * t130;
            let t136 = t15 * t12;
            let t137 = t136 * t114;
            let t139 = t135 * t137 / f64x8::splat(3.0);
            let t140 = t16 * t127;
            let t141 = t114 * param_d;
            let t144 = t140 * t141 * t130 / f64x8::splat(3.0);
            let t146 = f64x8::splat(1.0) / t19 / t120;
            let t147 = t18 * t146;
            let t149 = f64x8::splat(8.0) / f64x8::splat(3.0) * t147 * t30;
            let t151 = param_d * param_d;
            let t152 = t151 * t127;
            let t154 = f64x8::splat(1.0) / t19 / t3;
            let t157 = t24 * t130 - t152 * t154;
            let t158 = f64x8::splat(7.0) / f64x8::splat(3.0) * t157;
            let t159 = t7 * t158;
            let t161 = t124 * t27 / f64x8::splat(72.0) + t159 / f64x8::splat(72.0);
            let t166 = t48 * t40;
            let t167 = t38 - t119;
            let t168 = t166 * t167;
            let t170 = ((t41).select(f64x8::splat(0.0), f64x8::splat(8.0) / f64x8::splat(3.0) * t168));
            let t171 = t55 * t51;
            let t172 = -t167;
            let t173 = t171 * t172;
            let t175 = ((t52).select(f64x8::splat(0.0), f64x8::splat(8.0) / f64x8::splat(3.0) * t173));
            let t176 = t170 + t175;
            let t180 = t62 * t124;
            let t183 = t157 / f64x8::splat(54.0);
            let t184 = t183 * t80;
            let t186 = t63 * t184 / f64x8::splat(32.0);
            let t189 = f64x8::splat(1.0) / t68 / t66 / v_rho0;
            let t190 = v_sigma0 * t189;
            let t191 = t190 * t50;
            let t193 = t71 * t170;
            let t194 = t78 * t175;
            let t195 = -f64x8::splat(8.0) / f64x8::splat(3.0) * t191 + t193 + t194;
            let t196 = t65 * t195;
            let t202 = -t157 / f64x8::splat(3.0);
            let t203 = t202 * t95;
            let t205 = t63 * t203 / f64x8::splat(576.0);
            let t210 = ((t41).select(f64x8::splat(0.0), f64x8::splat(11.0) / f64x8::splat(3.0) * t49 * t167));
            let t214 = ((t52).select(f64x8::splat(0.0), f64x8::splat(11.0) / f64x8::splat(3.0) * t56 * t172));
            let t216 = -f64x8::splat(8.0) / f64x8::splat(3.0) * t190 * t89 + t71 * t210 + t78 * t214;
            let t217 = t84 * t216;
            let t225 = ((t41).select(f64x8::splat(0.0), f64x8::splat(2.0) * t40 * t167));
            let t226 = t225 * v_sigma2;
            let t229 = t77 * t175;
            let t234 = ((t52).select(f64x8::splat(0.0), f64x8::splat(2.0) * t51 * t172));
            let t235 = t234 * v_sigma0;
            let t238 = t189 * t50;
            let t241 = t70 * t170;
            let t247 = t149 - t22 * t161 - f64x8::splat(3.0) / f64x8::splat(20.0) * t37 * t124 * t58 - f64x8::splat(3.0) / f64x8::splat(20.0) * t37 * t7 * t176 + t180 * t81 / f64x8::splat(32.0) + t186 + t63 * t196 / f64x8::splat(32.0) + t180 * t96 / f64x8::splat(576.0) + t205 + t63 * t217 / f64x8::splat(576.0) - t62 * (-f64x8::splat(16.0) / f64x8::splat(9.0) * t191 + f64x8::splat(2.0) / f64x8::splat(3.0) * t193 + f64x8::splat(2.0) / f64x8::splat(3.0) * t194 - t226 * t103 / f64x8::splat(4.0) - t102 * t229 / f64x8::splat(4.0) - t235 * t108 / f64x8::splat(4.0) + f64x8::splat(2.0) / f64x8::splat(3.0) * t107 * t238 - t107 * t241 / f64x8::splat(4.0)) / f64x8::splat(8.0);
            let t250 = t16 * t12 * t247 - t124 * t12 - t133 + t139 + t144;
            let tvrho0 = t118 * t250 + tzk0;
            acc_vrho_0 = tvrho0;
            let t253 = f64x8::splat(2.0) * t119 + f64x8::splat(2.0) * t122;
            let t257 = t253 * t27 / f64x8::splat(72.0) + t159 / f64x8::splat(72.0);
            let t262 = -t38 - t119;
            let t263 = t166 * t262;
            let t265 = ((t41).select(f64x8::splat(0.0), f64x8::splat(8.0) / f64x8::splat(3.0) * t263));
            let t266 = -t262;
            let t267 = t171 * t266;
            let t269 = ((t52).select(f64x8::splat(0.0), f64x8::splat(8.0) / f64x8::splat(3.0) * t267));
            let t270 = t265 + t269;
            let t274 = t62 * t253;
            let t277 = t71 * t265;
            let t280 = f64x8::splat(1.0) / t75 / t73 / v_rho1;
            let t281 = v_sigma2 * t280;
            let t282 = t281 * t57;
            let t284 = t78 * t269;
            let t285 = t277 - f64x8::splat(8.0) / f64x8::splat(3.0) * t282 + t284;
            let t286 = t65 * t285;
            let t293 = ((t41).select(f64x8::splat(0.0), f64x8::splat(11.0) / f64x8::splat(3.0) * t49 * t262));
            let t299 = ((t52).select(f64x8::splat(0.0), f64x8::splat(11.0) / f64x8::splat(3.0) * t56 * t266));
            let t301 = t71 * t293 - f64x8::splat(8.0) / f64x8::splat(3.0) * t281 * t93 + t78 * t299;
            let t302 = t84 * t301;
            let t310 = ((t41).select(f64x8::splat(0.0), f64x8::splat(2.0) * t40 * t262));
            let t311 = t310 * v_sigma2;
            let t314 = t280 * t57;
            let t317 = t77 * t269;
            let t322 = ((t52).select(f64x8::splat(0.0), f64x8::splat(2.0) * t51 * t266));
            let t323 = t322 * v_sigma0;
            let t326 = t70 * t265;
            let t332 = t149 - t22 * t257 - f64x8::splat(3.0) / f64x8::splat(20.0) * t37 * t253 * t58 - f64x8::splat(3.0) / f64x8::splat(20.0) * t37 * t7 * t270 + t274 * t81 / f64x8::splat(32.0) + t186 + t63 * t286 / f64x8::splat(32.0) + t274 * t96 / f64x8::splat(576.0) + t205 + t63 * t302 / f64x8::splat(576.0) - t62 * (f64x8::splat(2.0) / f64x8::splat(3.0) * t277 - f64x8::splat(16.0) / f64x8::splat(9.0) * t282 + f64x8::splat(2.0) / f64x8::splat(3.0) * t284 - t311 * t103 / f64x8::splat(4.0) + f64x8::splat(2.0) / f64x8::splat(3.0) * t102 * t314 - t102 * t317 / f64x8::splat(4.0) - t323 * t108 / f64x8::splat(4.0) - t107 * t326 / f64x8::splat(4.0)) / f64x8::splat(8.0);
            let t335 = t16 * t12 * t332 - t253 * t12 - t133 + t139 + t144;
            let tvrho1 = t118 * t335 + tzk0;
            acc_vrho_1 = tvrho1;
            let t337 = t118 * param_b;
            let t338 = t21 * t30;
            let t339 = t65 * t70;
            let t340 = t339 * t50;
            let t343 = t84 * t70;
            let t344 = t343 * t89;
            let t348 = t106 * t70;
            let t354 = -t338 + t63 * t340 / f64x8::splat(32.0) + t63 * t344 / f64x8::splat(576.0) - t62 * (f64x8::splat(2.0) / f64x8::splat(3.0) * t108 - t348 * t50 / f64x8::splat(4.0)) / f64x8::splat(8.0);
            let t355 = t136 * t354;
            let tvsigma0 = t337 * t355;
            acc_vsigma_0 = tvsigma0;
            let t356 = t154 * param_a;
            let t357 = t356 * param_b;
            let t358 = t136 * t30;
            let tvsigma1 = -f64x8::splat(2.0) * t357 * t358;
            acc_vsigma_1 = tvsigma1;
            let t361 = t65 * t77;
            let t362 = t361 * t57;
            let t365 = t84 * t77;
            let t366 = t365 * t93;
            let t370 = t101 * t77;
            let t376 = -t338 + t63 * t362 / f64x8::splat(32.0) + t63 * t366 / f64x8::splat(576.0) - t62 * (f64x8::splat(2.0) / f64x8::splat(3.0) * t103 - t370 * t57 / f64x8::splat(4.0)) / f64x8::splat(8.0);
            let t377 = t136 * t376;
            let tvsigma2 = t337 * t377;
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
