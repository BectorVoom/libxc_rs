//! GGA_C_LM exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_lm.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_lm_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_lm_f: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let t1 = 1.0 / M_PI;
        let t2 = rho0 + rho1;
        let t3 = 1.0 / t2;
        let t6 = 1.0 + t1 * t3 / 36000.0;
        let t7 = M_CBRT3;
        let t8 = t7 * t7;
        let t9 = pow_1_3(t1);
        let t10 = 1.0 / t9;
        let t11 = t8 * t10;
        let t12 = M_CBRT4;
        let t13 = pow_1_3(t2);
        let t15 = t11 * t12 * t13;
        let t17 = 1.0 + 10.0 * t15;
        let t18 = rmath::ln(t17);
        let t20 = 0.0252 * t6 * t18;
        let t21 = t9 * t9;
        let t22 = t8 * t21;
        let t23 = t13 * t13;
        let t24 = 1.0 / t23;
        let t25 = t12 * t24;
        let t26 = t22 * t25;
        let t27 = 7e-06 * t26;
        let t28 = t7 * t9;
        let t29 = t12 * t12;
        let t32 = t28 * t29 / t13;
        let t33 = 0.000105 * t32;
        let t34 = rho0 - rho1;
        let t35 = t34 * t3;
        let t36 = 1.0 + t35;
        let t37 = t36 <= zeta_threshold;
        let t38 = pow_1_3(zeta_threshold);
        let t39 = t38 * zeta_threshold;
        let t40 = pow_1_3(t36);
        let t41 = t40 * t36;
        let t42 = piecewise3(t37, t39, t41);
        let t43 = 1.0 - t35;
        let t44 = t43 <= zeta_threshold;
        let t45 = pow_1_3(t43);
        let t46 = t45 * t43;
        let t47 = piecewise3(t44, t39, t46);
        let t49 = M_CBRT2;
        let t52 = 1.0 / (2.0 * t49 - 2.0);
        let t53 = (t42 + t47 - 2.0) * t52;
        let t55 = 1.0 + 5.658842421045167e-07 * t3;
        let t57 = 1.0 + 25.0 * t15;
        let t58 = rmath::ln(t57);
        let t63 = -0.0127 * t55 * t58 - 6.435555555555556e-06 * t26 + 8.383333333333333e-05 * t32 - 0.004166666666666667 + t20;
        let t64 = t53 * t63;
        let t65 = M_PI * t8;
        let t66 = M_PI * M_PI;
        let t67 = pow_1_3(t66);
        let t69 = 1.0 / t67 / t66;
        let t70 = rho0 * rho0;
        let t71 = pow_1_3(rho0);
        let t72 = t71 * t71;
        let t74 = 1.0 / t72 / t70;
        let t75 = sigma0 * t74;
        let t77 = rho1 * rho1;
        let t78 = pow_1_3(rho1);
        let t79 = t78 * t78;
        let t81 = 1.0 / t79 / t77;
        let t82 = sigma2 * t81;
        let t87 = t38 * t38;
        let t88 = t87 * zeta_threshold;
        let t89 = t40 * t40;
        let t90 = t89 * t36;
        let t91 = piecewise3(t37, t88, t90);
        let t92 = t45 * t45;
        let t93 = t92 * t43;
        let t94 = piecewise3(t44, t88, t93);
        let t95 = t91 + t94;
        let t96 = rmath::sqrt(t95);
        let t98 = M_SQRT2;
        let t99 = 1.0 / t96 * t98;
        let t100 = t7 * param_lm_f;
        let t101 = rmath::pow(t1, 1.0 / 6.0);
        let t102 = 1.0 / t101;
        let t104 = sigma0 + 2.0 * sigma1 + sigma2;
        let t105 = rmath::sqrt(t104);
        let t106 = t102 * t105;
        let t107 = rmath::pow(t2, 1.0 / 6.0);
        let t112 = rmath::exp(-t100 * t106 / t107 / t2);
        let t113 = t112 * t104;
        let t114 = t2 * t2;
        let t116 = 1.0 / t23 / t114;
        let t121 = t69 * (-7.0 / 36.0 * t49 * (t75 * t42 + t82 * t47) + 2.0 * t99 * t113 * t116);
        let t124 = t65 * t121 * t13 / 144.0;
        let tzk0 = -t20 + t27 - t33 + 0.0084 + t64 + t124;
        zk[ip] += tzk0;
    }
}
