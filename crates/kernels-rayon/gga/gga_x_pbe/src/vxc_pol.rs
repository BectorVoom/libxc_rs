//! GGA_X_PBE vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_pbe.c`
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
pub fn gga_x_pbe_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_kappa: f64,
    param_mu: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_kappa = f64x8::splat(param_kappa);
    let param_mu = f64x8::splat(param_mu);
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
            let t28 = f64x8::splat(M_CBRT6);
            let t29 = param_mu * t28;
            let t30 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t31 = (simd::cbrt(t30));
            let t32 = t31 * t31;
            let t33 = f64x8::splat(1.0) / t32;
            let t34 = t33 * v_sigma0;
            let t35 = v_rho0 * v_rho0;
            let t36 = (simd::cbrt(v_rho0));
            let t37 = t36 * t36;
            let t39 = f64x8::splat(1.0) / t37 / t35;
            let t43 = param_kappa + t29 * t34 * t39 / f64x8::splat(24.0);
            let t48 = f64x8::splat(1.0) + param_kappa * (f64x8::splat(1.0) - param_kappa / t43);
            let t52 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t48));
            let t53 = (v_rho1).simd_le(dens_threshold);
            let t54 = -t16;
            let t56 = ((t14).select(t11, (t10).select(t15, t54 * t7)));
            let t57 = f64x8::splat(1.0) + t56;
            let t58 = (t57).simd_le(zeta_threshold);
            let t59 = (simd::cbrt(t57));
            let t61 = ((t58).select(t22, t59 * t57));
            let t62 = t61 * t26;
            let t63 = t33 * v_sigma2;
            let t64 = v_rho1 * v_rho1;
            let t65 = (simd::cbrt(v_rho1));
            let t66 = t65 * t65;
            let t68 = f64x8::splat(1.0) / t66 / t64;
            let t72 = param_kappa + t29 * t63 * t68 / f64x8::splat(24.0);
            let t77 = f64x8::splat(1.0) + param_kappa * (f64x8::splat(1.0) - param_kappa / t72);
            let t81 = ((t53).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t62 * t77));
            let tzk0 = t52 + t81;
            acc_zk = tzk0;
            let t82 = t6 * t6;
            let t83 = f64x8::splat(1.0) / t82;
            let t84 = t16 * t83;
            let t86 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), t7 - t84)));
            let t89 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t86));
            let t90 = t89 * t26;
            let t94 = t26 * t26;
            let t95 = f64x8::splat(1.0) / t94;
            let t96 = t25 * t95;
            let t99 = t5 * t96 * t48 / f64x8::splat(8.0);
            let t100 = param_kappa * param_kappa;
            let t101 = t27 * t100;
            let t102 = t5 * t101;
            let t103 = t43 * t43;
            let t105 = f64x8::splat(1.0) / t103 * param_mu;
            let t106 = t105 * t28;
            let t107 = t35 * v_rho0;
            let t109 = f64x8::splat(1.0) / t37 / t107;
            let t111 = t106 * t34 * t109;
            let t115 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t90 * t48 - t99 + t102 * t111 / f64x8::splat(24.0)));
            let t116 = t54 * t83;
            let t118 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -t7 - t116)));
            let t121 = ((t58).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t59 * t118));
            let t122 = t121 * t26;
            let t126 = t61 * t95;
            let t129 = t5 * t126 * t77 / f64x8::splat(8.0);
            let t131 = ((t53).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t122 * t77 - t129));
            let tvrho0 = t52 + t81 + t6 * (t115 + t131);
            acc_vrho_0 = tvrho0;
            let t135 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -t7 - t84)));
            let t138 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t135));
            let t139 = t138 * t26;
            let t144 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t139 * t48 - t99));
            let t146 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), t7 - t116)));
            let t149 = ((t58).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t59 * t146));
            let t150 = t149 * t26;
            let t154 = t62 * t100;
            let t155 = t5 * t154;
            let t156 = t72 * t72;
            let t158 = f64x8::splat(1.0) / t156 * param_mu;
            let t159 = t158 * t28;
            let t160 = t64 * v_rho1;
            let t162 = f64x8::splat(1.0) / t66 / t160;
            let t164 = t159 * t63 * t162;
            let t168 = ((t53).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t150 * t77 - t129 + t155 * t164 / f64x8::splat(24.0)));
            let tvrho1 = t52 + t81 + t6 * (t144 + t168);
            acc_vrho_1 = tvrho1;
            let t171 = t28 * t33;
            let t173 = t105 * t171 * t39;
            let t176 = ((t1).select(f64x8::splat(0.0), -t102 * t173 / f64x8::splat(64.0)));
            let tvsigma0 = t6 * t176;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t178 = t158 * t171 * t68;
            let t181 = ((t53).select(f64x8::splat(0.0), -t155 * t178 / f64x8::splat(64.0)));
            let tvsigma2 = t6 * t181;
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
