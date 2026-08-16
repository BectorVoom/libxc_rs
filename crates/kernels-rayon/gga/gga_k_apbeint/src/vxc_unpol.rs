//! GGA_K_APBEINT vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_apbeint.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_k_apbeint_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_alpha: f64,
    param_kappa: f64,
    param_muGE: f64,
    param_muPBE: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = t3 * t3;
        let t5 = M_CBRTPI;
        let t7 = t4 * t5 * M_PI;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t15 = t14 * t14;
        let t17 = pow_1_3(t12);
        let t18 = t17 * t17;
        let t20 = piecewise3(t12 <= zeta_threshold, t15 * zeta_threshold, t18 * t12);
        let t21 = pow_1_3(rho[ip]);
        let t22 = t21 * t21;
        let t23 = t20 * t22;
        let t24 = param_muPBE - param_muGE;
        let t25 = t24 * param_alpha;
        let t26 = M_CBRT6;
        let t27 = M_PI * M_PI;
        let t28 = pow_1_3(t27);
        let t29 = t28 * t28;
        let t30 = 1.0 / t29;
        let t31 = t26 * t30;
        let t32 = t25 * t31;
        let t33 = M_CBRT2;
        let t34 = t33 * t33;
        let t35 = sigma[ip] * t34;
        let t36 = rho[ip] * rho[ip];
        let t38 = 1.0 / t22 / t36;
        let t41 = t35 * t38;
        let t44 = 1.0 + param_alpha * t26 * t30 * t41 / 24.0;
        let t45 = 1.0 / t44;
        let t46 = t38 * t45;
        let t51 = (param_muGE + t32 * t35 * t46 / 24.0) * t26;
        let t52 = t51 * t30;
        let t55 = param_kappa + t52 * t41 / 24.0;
        let t60 = 1.0 + param_kappa * (1.0 - param_kappa / t55);
        let t64 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t60);
        let tzk0 = 2.0 * t64;
        zk[ip] += tzk0;
        let t65 = 1.0 / t21;
        let t66 = t20 * t65;
        let t70 = t7 * t20;
        let t71 = param_kappa * param_kappa;
        let t72 = t22 * t71;
        let t73 = t55 * t55;
        let t74 = 1.0 / t73;
        let t75 = t36 * rho[ip];
        let t77 = 1.0 / t22 / t75;
        let t78 = t77 * t45;
        let t82 = param_alpha * param_alpha;
        let t83 = t24 * t82;
        let t84 = t26 * t26;
        let t86 = 1.0 / t28 / t27;
        let t87 = t84 * t86;
        let t88 = t83 * t87;
        let t89 = sigma[ip] * sigma[ip];
        let t90 = t89 * t33;
        let t91 = t36 * t36;
        let t92 = t91 * t36;
        let t94 = 1.0 / t21 / t92;
        let t95 = t44 * t44;
        let t96 = 1.0 / t95;
        let t97 = t94 * t96;
        let t102 = (-t32 * t35 * t78 / 9.0 + t88 * t90 * t97 / 108.0) * t26;
        let t103 = t102 * t30;
        let t106 = t35 * t77;
        let t109 = t103 * t41 / 24.0 - t52 * t106 / 9.0;
        let t110 = t74 * t109;
        let t115 = piecewise3(t2, 0.0, t7 * t66 * t60 / 10.0 + 3.0 / 20.0 * t70 * t72 * t110);
        let tvrho0 = 2.0 * rho[ip] * t115 + 2.0 * t64;
        vrho[ip] += tvrho0;
        let t118 = t25 * t26;
        let t119 = t30 * t34;
        let t124 = t91 * rho[ip];
        let t127 = 1.0 / t21 / t124 * t96;
        let t132 = (t118 * t119 * t46 / 24.0 - t88 * sigma[ip] * t33 * t127 / 288.0) * t26;
        let t133 = t132 * t30;
        let t135 = t119 * t38;
        let t138 = t133 * t41 / 24.0 + t51 * t135 / 24.0;
        let t139 = t74 * t138;
        let t143 = piecewise3(t2, 0.0, 3.0 / 20.0 * t70 * t72 * t139);
        let tvsigma0 = 2.0 * rho[ip] * t143;
        vsigma[ip] += tvsigma0;
    }
}
