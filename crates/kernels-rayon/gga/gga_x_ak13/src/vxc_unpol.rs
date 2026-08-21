//! GGA_X_AK13 vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_ak13.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_ak13_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_B1: f64,
    param_B2: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t7 = 1.0 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = piecewise5(t7, t8, t7, -t8, 0.0);
        let t11 = 1.0 + t10;
        let t13 = pow_1_3(zeta_threshold);
        let t15 = pow_1_3(t11);
        let t17 = piecewise3(t11 <= zeta_threshold, t13 * zeta_threshold, t15 * t11);
        let t18 = pow_1_3(rho[ip]);
        let t19 = t17 * t18;
        let t20 = M_CBRT6;
        let t21 = t20 * t20;
        let t23 = M_PI * M_PI;
        let t24 = pow_1_3(t23);
        let t25 = 1.0 / t24;
        let t26 = param_B1 * t21 * t25;
        let t27 = rmath::sqrt(sigma[ip]);
        let t28 = M_CBRT2;
        let t29 = t27 * t28;
        let t31 = 1.0 / t18 / rho[ip];
        let t32 = t21 * t25;
        let t36 = 1.0 + t32 * t29 * t31 / 12.0;
        let t37 = rmath::ln(t36);
        let t38 = t31 * t37;
        let t43 = param_B2 * t21 * t25;
        let t44 = 1.0 + t37;
        let t45 = rmath::ln(t44);
        let t46 = t31 * t45;
        let t50 = 1.0 + t26 * t29 * t38 / 12.0 + t43 * t29 * t46 / 12.0;
        let t54 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t50);
        let tzk0 = 2.0 * t54;
        zk[ip] += tzk0;
        let t55 = t18 * t18;
        let t57 = t17 / t55;
        let t61 = rho[ip] * rho[ip];
        let t63 = 1.0 / t18 / t61;
        let t64 = t63 * t37;
        let t69 = t24 * t24;
        let t70 = 1.0 / t69;
        let t71 = param_B1 * t20 * t70;
        let t72 = t28 * t28;
        let t73 = sigma[ip] * t72;
        let t74 = t61 * rho[ip];
        let t76 = 1.0 / t55 / t74;
        let t77 = 1.0 / t36;
        let t78 = t76 * t77;
        let t82 = t63 * t45;
        let t86 = param_B2 * t20;
        let t88 = t86 * t70 * sigma[ip];
        let t89 = t72 * t76;
        let t90 = 1.0 / t44;
        let t91 = t77 * t90;
        let t92 = t89 * t91;
        let t95 = -t26 * t29 * t64 / 9.0 - t71 * t73 * t78 / 18.0 - t43 * t29 * t82 / 9.0 - t88 * t92 / 18.0;
        let t100 = piecewise3(t2, 0.0, -t6 * t57 * t50 / 8.0 - 3.0 / 8.0 * t6 * t19 * t95);
        let tvrho0 = 2.0 * rho[ip] * t100 + 2.0 * t54;
        vrho[ip] += tvrho0;
        let t103 = 1.0 / t27;
        let t104 = t103 * t28;
        let t109 = 1.0 / t55 / t61;
        let t110 = t72 * t109;
        let t117 = t86 * t70;
        let t118 = t110 * t91;
        let t121 = t26 * t104 * t38 / 24.0 + t71 * t110 * t77 / 48.0 + t43 * t104 * t46 / 24.0 + t117 * t118 / 48.0;
        let t125 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t121);
        let tvsigma0 = 2.0 * rho[ip] * t125;
        vsigma[ip] += tvsigma0;
    }
}
