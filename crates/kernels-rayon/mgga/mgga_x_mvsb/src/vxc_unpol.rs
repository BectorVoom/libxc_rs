//! MGGA_X_MVSB vxc unpol kernel — explicit SIMD (bit-exact).
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

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_mvsb_vxc_unpol(
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
            let t7 = t4 / t5;
            let t8 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t9 = zeta_threshold - f64x8::splat(1.0);
            let t11 = ((t8).select(t9, (t8).select(-t9, f64x8::splat(0.0))));
            let t12 = f64x8::splat(1.0) + t11;
            let t14 = (simd::cbrt(zeta_threshold));
            let t16 = (simd::cbrt(t12));
            let t18 = (((t12).simd_le(zeta_threshold)).select(t14 * zeta_threshold, t16 * t12));
            let t19 = t7 * t18;
            let t20 = (simd::cbrt(v_rho));
            let t21 = f64x8::splat(M_CBRT2);
            let t22 = t21 * t21;
            let t23 = v_tau * t22;
            let t24 = t20 * t20;
            let t26 = f64x8::splat(1.0) / t24 / v_rho;
            let t27 = t23 * t26;
            let t28 = v_sigma * t22;
            let t29 = v_rho * v_rho;
            let t31 = f64x8::splat(1.0) / t24 / t29;
            let t34 = t27 - t28 * t31 / f64x8::splat(8.0);
            let t35 = f64x8::splat(M_CBRT6);
            let t36 = t35 * t35;
            let t37 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t38 = (simd::cbrt(t37));
            let t39 = t38 * t38;
            let t42 = t27 - f64x8::splat(3.0) / f64x8::splat(10.0) * t36 * t39;
            let t43 = f64x8::splat(1.0) / t42;
            let t46 = param_k0 * (-t34 * t43 + f64x8::splat(1.0));
            let t47 = t34 * t34;
            let t48 = param_e1 * t47;
            let t49 = t42 * t42;
            let t50 = f64x8::splat(1.0) / t49;
            let t52 = t48 * t50 + f64x8::splat(1.0);
            let t53 = t52 * t52;
            let t54 = t47 * t47;
            let t55 = param_c1 * t54;
            let t56 = t49 * t49;
            let t57 = f64x8::splat(1.0) / t56;
            let t59 = t55 * t57 + t53;
            let t60 = ((t59).sqrt().sqrt());
            let t61 = f64x8::splat(1.0) / t60;
            let t63 = t46 * t61 + f64x8::splat(1.0);
            let t67 = f64x8::splat(1.0) / t38 / t37;
            let t69 = v_sigma * v_sigma;
            let t71 = t29 * t29;
            let t72 = t71 * v_rho;
            let t74 = f64x8::splat(1.0) / t20 / t72;
            let t78 = f64x8::splat(1.0) + param_b * t36 * t67 * t69 * t21 * t74 / f64x8::splat(288.0);
            let t79 = (simd::pow(t78, f64x8::splat(1.0) / f64x8::splat(8.0)));
            let t80 = f64x8::splat(1.0) / t79;
            let t84 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t63 * t80));
            let tzk0 = f64x8::splat(2.0) * t84;
            acc_zk = tzk0;
            let t85 = f64x8::splat(1.0) / t24;
            let t90 = t23 * t31;
            let t92 = t29 * v_rho;
            let t94 = f64x8::splat(1.0) / t24 / t92;
            let t97 = -f64x8::splat(5.0) / f64x8::splat(3.0) * t90 + t28 * t94 / f64x8::splat(3.0);
            let t99 = t34 * t50;
            let t103 = param_k0 * (-t97 * t43 - f64x8::splat(5.0) / f64x8::splat(3.0) * t99 * t90);
            let t106 = f64x8::splat(1.0) / t60 / t59;
            let t107 = param_e1 * t34;
            let t108 = t50 * t97;
            let t111 = t49 * t42;
            let t112 = f64x8::splat(1.0) / t111;
            let t113 = t48 * t112;
            let t116 = f64x8::splat(2.0) * t107 * t108 + f64x8::splat(10.0) / f64x8::splat(3.0) * t113 * t90;
            let t120 = param_c1 * t47 * t34;
            let t121 = t57 * t97;
            let t125 = f64x8::splat(1.0) / t56 / t42;
            let t126 = t55 * t125;
            let t129 = f64x8::splat(2.0) * t52 * t116 + f64x8::splat(4.0) * t120 * t121 + f64x8::splat(20.0) / f64x8::splat(3.0) * t126 * t90;
            let t130 = t106 * t129;
            let t133 = t103 * t61 - t46 * t130 / f64x8::splat(4.0);
            let t138 = t71 * t29;
            let t139 = f64x8::splat(1.0) / t138;
            let t140 = t18 * t139;
            let t142 = t7 * t140 * t63;
            let t145 = f64x8::splat(1.0) / t79 / t78 * param_b;
            let t146 = t145 * t36;
            let t149 = t146 * t67 * t69 * t21;
            let t153 = ((t3).select(f64x8::splat(0.0), -t19 * t85 * t63 * t80 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t133 * t80 - t142 * t149 / f64x8::splat(1152.0)));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t153 + f64x8::splat(2.0) * t84;
            acc_vrho = tvrho0;
            let t156 = param_k0 * t22;
            let t157 = t31 * t43;
            let t161 = t52 * param_e1;
            let t162 = t161 * t34;
            let t163 = t50 * t22;
            let t164 = t163 * t31;
            let t166 = t57 * t22;
            let t167 = t166 * t31;
            let t168 = t120 * t167;
            let t170 = -t162 * t164 / f64x8::splat(2.0) - t168 / f64x8::splat(2.0);
            let t171 = t106 * t170;
            let t174 = t156 * t157 * t61 / f64x8::splat(8.0) - t46 * t171 / f64x8::splat(4.0);
            let t179 = f64x8::splat(1.0) / t72;
            let t180 = t18 * t179;
            let t182 = t7 * t180 * t63;
            let t185 = t146 * t67 * v_sigma * t21;
            let t189 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t174 * t80 + t182 * t185 / f64x8::splat(3072.0)));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t189;
            acc_vsigma = tvsigma0;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl = tvlapl0;
            let t191 = t22 * t26;
            let t195 = param_k0 * (-t191 * t43 + t99 * t191);
            let t197 = t163 * t26;
            let t199 = t112 * t22;
            let t200 = t199 * t26;
            let t203 = f64x8::splat(2.0) * t107 * t197 - f64x8::splat(2.0) * t48 * t200;
            let t206 = t166 * t26;
            let t209 = t125 * t22;
            let t213 = -f64x8::splat(4.0) * t55 * t209 * t26 + f64x8::splat(4.0) * t120 * t206 + f64x8::splat(2.0) * t52 * t203;
            let t214 = t106 * t213;
            let t217 = t195 * t61 - t46 * t214 / f64x8::splat(4.0);
            let t222 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t217 * t80));
            let tvtau0 = f64x8::splat(2.0) * v_rho * t222;
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
