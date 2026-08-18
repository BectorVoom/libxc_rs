//! GGA_X_NCAP exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_ncap.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_ncap_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_mu: f64,
    param_zeta: f64,
    param_alpha: f64,
    param_beta: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let t1 = rho0 <= dens_threshold;
        let t2 = M_CBRT3;
        let t3 = M_CBRTPI;
        let t5 = t2 / t3;
        let t6 = rho0 + rho1;
        let t7 = 1.0 / t6;
        let t10 = 2.0 * rho0 * t7 <= zeta_threshold;
        let t11 = zeta_threshold - 1.0;
        let t14 = 2.0 * rho1 * t7 <= zeta_threshold;
        let t15 = -t11;
        let t16 = rho0 - rho1;
        let t18 = piecewise5(t10, t11, t14, t15, t16 * t7);
        let t19 = 1.0 + t18;
        let t20 = t19 <= zeta_threshold;
        let t21 = pow_1_3(zeta_threshold);
        let t22 = t21 * zeta_threshold;
        let t23 = pow_1_3(t19);
        let t25 = piecewise3(t20, t22, t23 * t19);
        let t26 = pow_1_3(t6);
        let t27 = t25 * t26;
        let t28 = M_CBRT6;
        let t29 = t28 * t28;
        let t30 = M_PI * M_PI;
        let t31 = pow_1_3(t30);
        let t32 = 1.0 / t31;
        let t33 = t29 * t32;
        let t34 = f64::sqrt(sigma0);
        let t35 = pow_1_3(rho0);
        let t37 = 1.0 / t35 / rho0;
        let t38 = t34 * t37;
        let t40 = t33 * t38 / 12.0;
        let t41 = f64::tanh(t40);
        let t42 = param_mu * t41;
        let t43 = f64::ln(t40 + f64::sqrt(t40 * t40 + 1.0));
        let t44 = 1.0 - param_zeta;
        let t46 = t44 * t29 * t32;
        let t47 = 1.0 + t40;
        let t48 = f64::ln(t47);
        let t51 = param_zeta * t29;
        let t52 = t32 * t34;
        let t58 = 1.0 + param_alpha * (t51 * t52 * t37 / 12.0 + t46 * t38 * t48 / 12.0);
        let t59 = t43 * t58;
        let t60 = param_beta * t41;
        let t62 = t60 * t43 + 1.0;
        let t63 = 1.0 / t62;
        let t64 = t59 * t63;
        let t66 = t42 * t64 + 1.0;
        let t70 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t66);
        let t71 = rho1 <= dens_threshold;
        let t72 = -t16;
        let t74 = piecewise5(t14, t11, t10, t15, t72 * t7);
        let t75 = 1.0 + t74;
        let t76 = t75 <= zeta_threshold;
        let t77 = pow_1_3(t75);
        let t79 = piecewise3(t76, t22, t77 * t75);
        let t80 = t79 * t26;
        let t81 = f64::sqrt(sigma2);
        let t82 = pow_1_3(rho1);
        let t84 = 1.0 / t82 / rho1;
        let t85 = t81 * t84;
        let t87 = t33 * t85 / 12.0;
        let t88 = f64::tanh(t87);
        let t89 = param_mu * t88;
        let t90 = f64::ln(t87 + f64::sqrt(t87 * t87 + 1.0));
        let t91 = 1.0 + t87;
        let t92 = f64::ln(t91);
        let t95 = t32 * t81;
        let t101 = 1.0 + param_alpha * (t46 * t85 * t92 / 12.0 + t51 * t95 * t84 / 12.0);
        let t102 = t90 * t101;
        let t103 = param_beta * t88;
        let t105 = t103 * t90 + 1.0;
        let t106 = 1.0 / t105;
        let t107 = t102 * t106;
        let t109 = t89 * t107 + 1.0;
        let t113 = piecewise3(t71, 0.0, -3.0 / 8.0 * t5 * t80 * t109);
        let tzk0 = t70 + t113;
        zk[ip] += tzk0;
    }
}
