//! GGA_X_B86 vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_b86.c`
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
pub fn gga_x_b86_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_beta: f64,
    param_gamma: f64,
    param_omega: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_beta = f64x8::splat(param_beta);
    let param_gamma = f64x8::splat(param_gamma);
    let param_omega = f64x8::splat(param_omega);
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
            let t1 = (v_rho0).simd_le(dens_threshold);
            let t2 = f64x8::splat(M_CBRT3);
            let t3 = f64x8::splat(M_CBRTPI);
            let t5 = t2 / t3;
            let t6 = v_rho0 + v_rho1;
            let t7 = f64x8::splat(1.0) / t6;
            let t10 = (f64x8::splat(2.0) * v_rho0 * t7).simd_le(zeta_threshold);
            let t11 = zeta_threshold - f64x8::splat(1.0);
            let t14 = (f64x8::splat(2.0) * v_rho1 * t7).simd_le(zeta_threshold);
            let t15 = -t11;
            let t16 = v_rho0 - v_rho1;
            let t18 = ((t10).select(t11, (t14).select(t15, t16 * t7)));
            let t19 = f64x8::splat(1.0) + t18;
            let t20 = (t19).simd_le(zeta_threshold);
            let t21 = (simd::cbrt(zeta_threshold));
            let t22 = t21 * zeta_threshold;
            let t23 = (simd::cbrt(t19));
            let t25 = ((t20).select(t22, t23 * t19));
            let t26 = (simd::cbrt(t6));
            let t27 = t25 * t26;
            let t28 = param_beta * v_sigma0;
            let t29 = v_rho0 * v_rho0;
            let t30 = (simd::cbrt(v_rho0));
            let t31 = t30 * t30;
            let t33 = f64x8::splat(1.0) / t31 / t29;
            let t36 = param_gamma * v_sigma0 * t33 + f64x8::splat(1.0);
            let t37 = (simd::pow(t36, param_omega));
            let t38 = f64x8::splat(1.0) / t37;
            let t41 = t28 * t33 * t38 + f64x8::splat(1.0);
            let t45 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t41));
            let t46 = (v_rho1).simd_le(dens_threshold);
            let t47 = -t16;
            let t49 = ((t14).select(t11, (t10).select(t15, t47 * t7)));
            let t50 = f64x8::splat(1.0) + t49;
            let t51 = (t50).simd_le(zeta_threshold);
            let t52 = (simd::cbrt(t50));
            let t54 = ((t51).select(t22, t52 * t50));
            let t55 = t54 * t26;
            let t56 = param_beta * v_sigma2;
            let t57 = v_rho1 * v_rho1;
            let t58 = (simd::cbrt(v_rho1));
            let t59 = t58 * t58;
            let t61 = f64x8::splat(1.0) / t59 / t57;
            let t64 = param_gamma * v_sigma2 * t61 + f64x8::splat(1.0);
            let t65 = (simd::pow(t64, param_omega));
            let t66 = f64x8::splat(1.0) / t65;
            let t69 = t56 * t61 * t66 + f64x8::splat(1.0);
            let t73 = ((t46).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t55 * t69));
            let tzk0 = t45 + t73;
            acc_zk = tzk0;
            let t74 = t6 * t6;
            let t75 = f64x8::splat(1.0) / t74;
            let t76 = t16 * t75;
            let t78 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), t7 - t76)));
            let t81 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t78));
            let t82 = t81 * t26;
            let t86 = t26 * t26;
            let t87 = f64x8::splat(1.0) / t86;
            let t88 = t25 * t87;
            let t91 = t5 * t88 * t41 / f64x8::splat(8.0);
            let t92 = t29 * v_rho0;
            let t94 = f64x8::splat(1.0) / t31 / t92;
            let t97 = v_sigma0 * v_sigma0;
            let t98 = param_beta * t97;
            let t99 = t29 * t29;
            let t100 = t99 * t29;
            let t102 = f64x8::splat(1.0) / t30 / t100;
            let t104 = t38 * param_omega;
            let t105 = f64x8::splat(1.0) / t36;
            let t107 = t104 * param_gamma * t105;
            let t110 = f64x8::splat(8.0) / f64x8::splat(3.0) * t98 * t102 * t107 - f64x8::splat(8.0) / f64x8::splat(3.0) * t28 * t94 * t38;
            let t115 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t82 * t41 - t91 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t110));
            let t116 = t47 * t75;
            let t118 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -t7 - t116)));
            let t121 = ((t51).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t52 * t118));
            let t122 = t121 * t26;
            let t126 = t54 * t87;
            let t129 = t5 * t126 * t69 / f64x8::splat(8.0);
            let t131 = ((t46).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t122 * t69 - t129));
            let tvrho0 = t45 + t73 + t6 * (t115 + t131);
            acc_vrho_0 = tvrho0;
            let t135 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -t7 - t76)));
            let t138 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t135));
            let t139 = t138 * t26;
            let t144 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t139 * t41 - t91));
            let t146 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), t7 - t116)));
            let t149 = ((t51).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t52 * t146));
            let t150 = t149 * t26;
            let t154 = t57 * v_rho1;
            let t156 = f64x8::splat(1.0) / t59 / t154;
            let t159 = v_sigma2 * v_sigma2;
            let t160 = param_beta * t159;
            let t161 = t57 * t57;
            let t162 = t161 * t57;
            let t164 = f64x8::splat(1.0) / t58 / t162;
            let t166 = t66 * param_omega;
            let t167 = f64x8::splat(1.0) / t64;
            let t169 = t166 * param_gamma * t167;
            let t172 = -f64x8::splat(8.0) / f64x8::splat(3.0) * t56 * t156 * t66 + f64x8::splat(8.0) / f64x8::splat(3.0) * t160 * t164 * t169;
            let t177 = ((t46).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t150 * t69 - t129 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t55 * t172));
            let tvrho1 = t45 + t73 + t6 * (t144 + t177);
            acc_vrho_1 = tvrho1;
            let t182 = t99 * v_rho0;
            let t184 = f64x8::splat(1.0) / t30 / t182;
            let t187 = -t28 * t184 * t107 + param_beta * t33 * t38;
            let t191 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t187));
            let tvsigma0 = t6 * t191;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t194 = t161 * v_rho1;
            let t196 = f64x8::splat(1.0) / t58 / t194;
            let t199 = -t56 * t196 * t169 + param_beta * t61 * t66;
            let t203 = ((t46).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t55 * t199));
            let tvsigma2 = t6 * t203;
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
