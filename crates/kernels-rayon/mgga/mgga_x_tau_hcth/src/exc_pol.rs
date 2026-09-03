//! MGGA_X_TAU_HCTH exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_tau_hcth.c`
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
pub fn mgga_x_tau_hcth_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_cx_local_1: f64,
    param_cx_local_2: f64,
    param_cx_local_3: f64,
    param_cx_nlocal_1: f64,
    param_cx_nlocal_2: f64,
    param_cx_nlocal_3: f64,
    param_cx_nlocal_0: f64,
    param_cx_local_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_cx_local_1 = f64x8::splat(param_cx_local_1);
    let param_cx_local_2 = f64x8::splat(param_cx_local_2);
    let param_cx_local_3 = f64x8::splat(param_cx_local_3);
    let param_cx_nlocal_1 = f64x8::splat(param_cx_nlocal_1);
    let param_cx_nlocal_2 = f64x8::splat(param_cx_nlocal_2);
    let param_cx_nlocal_3 = f64x8::splat(param_cx_nlocal_3);
    let param_cx_nlocal_0 = f64x8::splat(param_cx_nlocal_0);
    let param_cx_local_0 = f64x8::splat(param_cx_local_0);
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
            let t19 = ((t11).select(t12, (t15).select(t16, t17 * t8)));
            let t20 = f64x8::splat(1.0) + t19;
            let t21 = (t20).simd_le(zeta_threshold);
            let t22 = (simd::cbrt(zeta_threshold));
            let t23 = t22 * zeta_threshold;
            let t24 = (simd::cbrt(t20));
            let t26 = ((t21).select(t23, t24 * t20));
            let t27 = (simd::cbrt(t7));
            let t28 = t26 * t27;
            let t29 = param_cx_local_0;
            let t30 = param_cx_local_1;
            let t31 = t30 * v_sigma0;
            let t32 = v_rho0 * v_rho0;
            let t33 = (simd::cbrt(v_rho0));
            let t34 = t33 * t33;
            let t36 = f64x8::splat(1.0) / t34 / t32;
            let t39 = f64x8::splat(1.0) + f64x8::splat(0.004) * v_sigma0 * t36;
            let t40 = f64x8::splat(1.0) / t39;
            let t41 = t36 * t40;
            let t44 = param_cx_local_2;
            let t45 = v_sigma0 * v_sigma0;
            let t46 = t44 * t45;
            let t47 = t32 * t32;
            let t48 = t47 * v_rho0;
            let t50 = f64x8::splat(1.0) / t33 / t48;
            let t51 = t39 * t39;
            let t52 = f64x8::splat(1.0) / t51;
            let t53 = t50 * t52;
            let t56 = param_cx_local_3;
            let t57 = t45 * v_sigma0;
            let t58 = t56 * t57;
            let t59 = t47 * t47;
            let t60 = f64x8::splat(1.0) / t59;
            let t61 = t51 * t39;
            let t62 = f64x8::splat(1.0) / t61;
            let t63 = t60 * t62;
            let t66 = param_cx_nlocal_0;
            let t67 = param_cx_nlocal_1;
            let t68 = t67 * v_sigma0;
            let t71 = param_cx_nlocal_2;
            let t72 = t71 * t45;
            let t75 = param_cx_nlocal_3;
            let t76 = t75 * t57;
            let t79 = t66 + f64x8::splat(0.004) * t68 * t41 + f64x8::splat(1.6e-05) * t72 * t53 + f64x8::splat(6.4e-08) * t76 * t63;
            let t80 = f64x8::splat(M_CBRT6);
            let t81 = t80 * t80;
            let t82 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t83 = (simd::cbrt(t82));
            let t84 = t83 * t83;
            let t86 = f64x8::splat(3.0) / f64x8::splat(10.0) * t81 * t84;
            let t88 = f64x8::splat(1.0) / t34 / v_rho0;
            let t89 = v_tau0 * t88;
            let t90 = t86 - t89;
            let t91 = t86 + t89;
            let t92 = f64x8::splat(1.0) / t91;
            let t94 = t90 * t90;
            let t95 = t94 * t90;
            let t96 = t91 * t91;
            let t97 = t96 * t91;
            let t98 = f64x8::splat(1.0) / t97;
            let t101 = t94 * t94;
            let t102 = t101 * t90;
            let t103 = t96 * t96;
            let t105 = f64x8::splat(1.0) / t103 / t91;
            let t107 = t102 * t105 + t90 * t92 - f64x8::splat(2.0) * t95 * t98;
            let t109 = t29 + f64x8::splat(0.004) * t31 * t41 + f64x8::splat(1.6e-05) * t46 * t53 + f64x8::splat(6.4e-08) * t58 * t63 + t79 * t107;
            let t113 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t109));
            let t114 = (v_rho1).simd_le(dens_threshold);
            let t115 = -t17;
            let t117 = ((t15).select(t12, (t11).select(t16, t115 * t8)));
            let t118 = f64x8::splat(1.0) + t117;
            let t119 = (t118).simd_le(zeta_threshold);
            let t120 = (simd::cbrt(t118));
            let t122 = ((t119).select(t23, t120 * t118));
            let t123 = t122 * t27;
            let t124 = t30 * v_sigma2;
            let t125 = v_rho1 * v_rho1;
            let t126 = (simd::cbrt(v_rho1));
            let t127 = t126 * t126;
            let t129 = f64x8::splat(1.0) / t127 / t125;
            let t132 = f64x8::splat(1.0) + f64x8::splat(0.004) * v_sigma2 * t129;
            let t133 = f64x8::splat(1.0) / t132;
            let t134 = t129 * t133;
            let t137 = v_sigma2 * v_sigma2;
            let t138 = t44 * t137;
            let t139 = t125 * t125;
            let t140 = t139 * v_rho1;
            let t142 = f64x8::splat(1.0) / t126 / t140;
            let t143 = t132 * t132;
            let t144 = f64x8::splat(1.0) / t143;
            let t145 = t142 * t144;
            let t148 = t137 * v_sigma2;
            let t149 = t56 * t148;
            let t150 = t139 * t139;
            let t151 = f64x8::splat(1.0) / t150;
            let t152 = t143 * t132;
            let t153 = f64x8::splat(1.0) / t152;
            let t154 = t151 * t153;
            let t157 = t67 * v_sigma2;
            let t160 = t71 * t137;
            let t163 = t75 * t148;
            let t166 = t66 + f64x8::splat(0.004) * t157 * t134 + f64x8::splat(1.6e-05) * t160 * t145 + f64x8::splat(6.4e-08) * t163 * t154;
            let t168 = f64x8::splat(1.0) / t127 / v_rho1;
            let t169 = v_tau1 * t168;
            let t170 = t86 - t169;
            let t171 = t86 + t169;
            let t172 = f64x8::splat(1.0) / t171;
            let t174 = t170 * t170;
            let t175 = t174 * t170;
            let t176 = t171 * t171;
            let t177 = t176 * t171;
            let t178 = f64x8::splat(1.0) / t177;
            let t181 = t174 * t174;
            let t182 = t181 * t170;
            let t183 = t176 * t176;
            let t185 = f64x8::splat(1.0) / t183 / t171;
            let t187 = t170 * t172 - f64x8::splat(2.0) * t175 * t178 + t182 * t185;
            let t189 = t29 + f64x8::splat(0.004) * t124 * t134 + f64x8::splat(1.6e-05) * t138 * t145 + f64x8::splat(6.4e-08) * t149 * t154 + t166 * t187;
            let t193 = ((t114).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t123 * t189));
            let tzk0 = t113 + t193;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
