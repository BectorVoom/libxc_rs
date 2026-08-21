//! GGA_XC_TH1 exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_xc_th1.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_xc_th1_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
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
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let t1 = param_omega_0;
        let t2 = rmath::pow(rho0, 1.0 / 6.0);
        let t3 = t2 * rho0;
        let t4 = rmath::pow(rho1, 1.0 / 6.0);
        let t5 = t4 * rho1;
        let t6 = t3 + t5;
        let t8 = param_omega_1;
        let t9 = pow_1_3(rho0);
        let t10 = t9 * rho0;
        let t11 = pow_1_3(rho1);
        let t12 = t11 * rho1;
        let t13 = t10 + t12;
        let t15 = param_omega_2;
        let t16 = rmath::sqrt(rho0);
        let t17 = t16 * rho0;
        let t18 = rmath::sqrt(rho1);
        let t19 = t18 * rho1;
        let t20 = t17 + t19;
        let t22 = param_omega_3;
        let t23 = t9 * t9;
        let t24 = t23 * rho0;
        let t25 = t11 * t11;
        let t26 = t25 * rho1;
        let t27 = t24 + t26;
        let t29 = param_omega_4;
        let t30 = t29 * t13;
        let t31 = rmath::sqrt(sigma0);
        let t32 = 1.0 / t10;
        let t33 = t31 * t32;
        let t34 = rho0 - rho1;
        let t35 = rho0 + rho1;
        let t36 = 1.0 / t35;
        let t37 = t34 * t36;
        let t38 = 1.0 + t37;
        let t39 = t38 <= zeta_threshold;
        let t40 = pow_1_3(zeta_threshold);
        let t41 = t40 * zeta_threshold;
        let t42 = pow_1_3(t38);
        let t44 = piecewise3(t39, t41, t42 * t38);
        let t45 = M_CBRT2;
        let t46 = t45 * t45;
        let t47 = t44 * t46;
        let t49 = rmath::sqrt(sigma2);
        let t50 = 1.0 / t12;
        let t51 = t49 * t50;
        let t52 = 1.0 - t37;
        let t53 = t52 <= zeta_threshold;
        let t54 = pow_1_3(t52);
        let t56 = piecewise3(t53, t41, t54 * t52);
        let t57 = t56 * t46;
        let t60 = t33 * t47 / 4.0 + t51 * t57 / 4.0;
        let t63 = param_omega_5;
        let t64 = t63 * t20;
        let t67 = param_omega_6;
        let t68 = t67 * t27;
        let t71 = param_omega_7;
        let t72 = t2 * t2;
        let t73 = t72 * t72;
        let t74 = t73 * t2;
        let t75 = t74 * rho0;
        let t76 = t4 * t4;
        let t77 = t76 * t76;
        let t78 = t77 * t4;
        let t79 = t78 * rho1;
        let t80 = t75 + t79;
        let t81 = t71 * t80;
        let t84 = param_omega_8;
        let t85 = t84 * t20;
        let t86 = rho0 * rho0;
        let t88 = 1.0 / t23 / t86;
        let t89 = sigma0 * t88;
        let t90 = t44 * t44;
        let t91 = t90 * t45;
        let t92 = t89 * t91;
        let t93 = rho1 * rho1;
        let t95 = 1.0 / t25 / t93;
        let t96 = sigma2 * t95;
        let t97 = t56 * t56;
        let t98 = t97 * t45;
        let t99 = t96 * t98;
        let t101 = t92 / 8.0 + t99 / 8.0;
        let t104 = param_omega_9;
        let t105 = t104 * t27;
        let t109 = param_omega_10;
        let t110 = t109 * t80;
        let t113 = param_omega_11;
        let t114 = t86 + t93;
        let t115 = t113 * t114;
        let t118 = param_omega_12;
        let t119 = t118 * t20;
        let t123 = sigma0 + 2.0 * sigma1 + sigma2;
        let t124 = t35 * t35;
        let t125 = pow_1_3(t35);
        let t126 = t125 * t125;
        let t128 = 1.0 / t126 / t124;
        let t130 = t92 / 4.0 + t99 / 4.0 - t123 * t128;
        let t132 = param_omega_13;
        let t133 = t132 * t27;
        let t135 = param_omega_14;
        let t136 = t135 * t80;
        let t138 = param_omega_15;
        let t139 = t138 * t114;
        let t141 = param_omega_16;
        let t142 = t141 * t6;
        let t143 = t34 * t34;
        let t144 = 1.0 / t124;
        let t145 = t143 * t144;
        let t147 = param_omega_17;
        let t148 = t147 * t13;
        let t150 = param_omega_18;
        let t151 = t150 * t20;
        let t153 = param_omega_19;
        let t154 = t153 * t27;
        let t156 = param_omega_20;
        let t158 = t110 * t101 / 2.0 + t115 * t101 / 2.0 + t119 * t130 + t133 * t130 + t136 * t130 + t139 * t130 + t142 * t145 + t148 * t145 + t151 * t145 + t154 * t145 + t156 * t35;
        let tzk0 = (t1 * t6 + t8 * t13 + t15 * t20 + t22 * t27 + t30 * t60 / 2.0 + t64 * t60 / 2.0 + t68 * t60 / 2.0 + t81 * t60 / 2.0 + t85 * t101 / 2.0 + t105 * t101 / 2.0 + t158) * t36;
        zk[ip] += tzk0;
    }
}
