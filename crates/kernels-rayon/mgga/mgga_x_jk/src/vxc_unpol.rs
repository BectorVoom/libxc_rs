//! MGGA_X_JK vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_jk.c`
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

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_jk_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    param_beta: f64,
    param_gamma: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_beta = f64x8::splat(param_beta);
    let param_gamma = f64x8::splat(param_gamma);
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
        let mut acc_vrho = V_ZERO;
        let mut acc_vsigma = V_ZERO;
        let mut acc_vlapl = V_ZERO;
        let mut acc_vtau = V_ZERO;
        {
            let t3 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t4 = f64x8::splat(M_CBRT3);
            let t5 = f64x8::splat(M_CBRTPI);
            let t6 = f64x8::splat(1.0) / t5;
            let t7 = t4 * t6;
            let t8 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t9 = zeta_threshold - f64x8::splat(1.0);
            let t11 = ((t8).select(t9, (t8).select(-t9, f64x8::splat(0.0))));
            let t12 = f64x8::splat(1.0) + t11;
            let t14 = (simd::cbrt(zeta_threshold));
            let t16 = (simd::cbrt(t12));
            let t18 = (((t12).simd_le(zeta_threshold)).select(t14 * zeta_threshold, t16 * t12));
            let t19 = (simd::cbrt(v_rho));
            let t20 = t18 * t19;
            let t21 = t4 * t4;
            let t22 = param_beta * t21;
            let t24 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t25 = f64x8::splat(1.0) / t24;
            let t26 = f64x8::splat(M_CBRT4);
            let t27 = t25 * t26;
            let t28 = t22 * t27;
            let t29 = f64x8::splat(M_CBRT2);
            let t30 = t29 * t29;
            let t31 = v_sigma * t30;
            let t32 = v_rho * v_rho;
            let t33 = t19 * t19;
            let t34 = t33 * t32;
            let t35 = f64x8::splat(1.0) / t34;
            let t36 = param_gamma * param_beta;
            let t37 = ((v_sigma).sqrt());
            let t38 = t36 * t37;
            let t40 = f64x8::splat(1.0) / t19 / v_rho;
            let t41 = t29 * t40;
            let t44 = (simd::ln(t37 * t29 * t40 + ((((t37 * t29 * t40) * (t37 * t29 * t40)) + f64x8::splat(1.0)).sqrt())));
            let t45 = t41 * t44;
            let t47 = t38 * t45 + f64x8::splat(1.0);
            let t48 = f64x8::splat(1.0) / t47;
            let t49 = t35 * t48;
            let t50 = t31 * t35;
            let t51 = v_lapl * t30;
            let t52 = t33 * v_rho;
            let t53 = f64x8::splat(1.0) / t52;
            let t55 = -t51 * t53 + t50;
            let t56 = f64x8::splat(1.0) / v_sigma;
            let t57 = t55 * t56;
            let t58 = t29 * t34;
            let t60 = t57 * t58 + f64x8::splat(1.0);
            let t61 = f64x8::splat(1.0) / t60;
            let t66 = f64x8::splat(1.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t28 * t31 * t49 * t61;
            let t70 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t66));
            let tzk0 = f64x8::splat(2.0) * t70;
            acc_zk = tzk0;
            let t72 = t18 / t33;
            let t76 = t32 * v_rho;
            let t78 = f64x8::splat(1.0) / t33 / t76;
            let t79 = t78 * t48;
            let t85 = t22 * t27 * v_sigma;
            let t86 = t30 * t35;
            let t87 = t47 * t47;
            let t88 = f64x8::splat(1.0) / t87;
            let t89 = t88 * t61;
            let t91 = f64x8::splat(1.0) / t19 / t32;
            let t93 = t29 * t91 * t44;
            let t95 = t36 * v_sigma;
            let t96 = t30 * t78;
            let t97 = t50 + f64x8::splat(1.0);
            let t98 = ((t97).sqrt());
            let t99 = f64x8::splat(1.0) / t98;
            let t100 = t96 * t99;
            let t103 = -f64x8::splat(4.0) / f64x8::splat(3.0) * t95 * t100 - f64x8::splat(4.0) / f64x8::splat(3.0) * t38 * t93;
            let t104 = t89 * t103;
            let t105 = t86 * t104;
            let t108 = t60 * t60;
            let t109 = f64x8::splat(1.0) / t108;
            let t110 = t48 * t109;
            let t115 = -f64x8::splat(8.0) / f64x8::splat(3.0) * t31 * t78 + f64x8::splat(5.0) / f64x8::splat(3.0) * t51 * t35;
            let t116 = t115 * t56;
            let t118 = t29 * t52;
            let t121 = t116 * t58 + f64x8::splat(8.0) / f64x8::splat(3.0) * t57 * t118;
            let t122 = t110 * t121;
            let t123 = t86 * t122;
            let t126 = -f64x8::splat(16.0) / f64x8::splat(27.0) * t28 * t31 * t79 * t61 - f64x8::splat(2.0) / f64x8::splat(9.0) * t85 * t105 - f64x8::splat(2.0) / f64x8::splat(9.0) * t85 * t123;
            let t131 = ((t3).select(f64x8::splat(0.0), -t7 * t72 * t66 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t126));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t131 + f64x8::splat(2.0) * t70;
            acc_vrho = tvrho0;
            let t134 = t48 * t61;
            let t138 = t36 / t37;
            let t140 = t86 * t99;
            let t143 = t138 * t45 / f64x8::splat(2.0) + t36 * t140 / f64x8::splat(2.0);
            let t144 = t89 * t143;
            let t145 = t86 * t144;
            let t148 = v_sigma * v_sigma;
            let t149 = f64x8::splat(1.0) / t148;
            let t150 = t55 * t149;
            let t152 = -t150 * t58 + f64x8::splat(2.0) * t56;
            let t153 = t110 * t152;
            let t154 = t86 * t153;
            let t157 = f64x8::splat(2.0) / f64x8::splat(9.0) * t28 * t86 * t134 - f64x8::splat(2.0) / f64x8::splat(9.0) * t85 * t145 - f64x8::splat(2.0) / f64x8::splat(9.0) * t85 * t154;
            let t161 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t157));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t161;
            acc_vsigma = tvsigma0;
            let t163 = t6 * t18;
            let t164 = t40 * param_beta;
            let t166 = t30 * t48;
            let t168 = t27 * t166 * t109;
            let t171 = ((t3).select(f64x8::splat(0.0), -t163 * t164 * t168 / f64x8::splat(2.0)));
            let tvlapl0 = f64x8::splat(2.0) * v_rho * t171;
            acc_vlapl = tvlapl0;
            let tvtau0 = f64x8::splat(0.0);
            acc_vtau = tvtau0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vlapl.into(); vlapl[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vtau.into(); vtau[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
