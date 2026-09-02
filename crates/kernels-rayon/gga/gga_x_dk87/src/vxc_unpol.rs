//! GGA_X_DK87 vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_dk87.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_dk87_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_alpha: f64,
    param_a1: f64,
    param_b1: f64,
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
        let t20 = 1.0 / M_PI;
        let t21 = M_CBRT6;
        let t22 = t21 * t21;
        let t23 = t20 * t22;
        let t24 = M_PI * M_PI;
        let t25 = pow_1_3(t24);
        let t26 = 1.0 / t25;
        let t27 = t3 * t3;
        let t29 = pow_1_3(t20);
        let t30 = 1.0 / t29;
        let t32 = t23 * t26 * t27 * t30;
        let t33 = M_CBRT4;
        let t34 = t33 * sigma[ip];
        let t35 = M_CBRT2;
        let t36 = t35 * t35;
        let t37 = t34 * t36;
        let t38 = rho[ip] * rho[ip];
        let t39 = t18 * t18;
        let t41 = 1.0 / t39 / t38;
        let t42 = rmath::sqrt(sigma[ip]);
        let t47 = rmath::pow(t42 * t35 / t18 / rho[ip], param_alpha);
        let t48 = param_a1 * t47;
        let t49 = 1.0 + t48;
        let t51 = param_b1 * sigma[ip];
        let t52 = t36 * t41;
        let t54 = t51 * t52 + 1.0;
        let t55 = 1.0 / t54;
        let t56 = t41 * t49 * t55;
        let t60 = 1.0 + 7.0 / 11664.0 * t32 * t37 * t56;
        let t64 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t60);
        let tzk0 = 2.0 * t64;
        zk[ip] += tzk0;
        let t66 = t17 / t39;
        let t70 = t38 * rho[ip];
        let t72 = 1.0 / t39 / t70;
        let t74 = t72 * t49 * t55;
        let t78 = t23 * t26;
        let t79 = t27 * t30;
        let t81 = t78 * t79 * t33;
        let t82 = sigma[ip] * t36;
        let t85 = t48 * param_alpha * t55;
        let t89 = sigma[ip] * sigma[ip];
        let t90 = t89 * t35;
        let t91 = t38 * t38;
        let t92 = t91 * t38;
        let t94 = 1.0 / t18 / t92;
        let t96 = t54 * t54;
        let t97 = 1.0 / t96;
        let t99 = t49 * t97 * param_b1;
        let t103 = -7.0 / 4374.0 * t32 * t37 * t74 - 7.0 / 8748.0 * t81 * t82 * t72 * t85 + 7.0 / 2187.0 * t81 * t90 * t94 * t99;
        let t108 = piecewise3(t2, 0.0, -t6 * t66 * t60 / 8.0 - 3.0 / 8.0 * t6 * t19 * t103);
        let tvrho0 = 2.0 * rho[ip] * t108 + 2.0 * t64;
        vrho[ip] += tvrho0;
        let t111 = t33 * t36;
        let t115 = t52 * param_a1;
        let t116 = t47 * param_alpha;
        let t117 = t116 * t55;
        let t122 = t91 * rho[ip];
        let t124 = 1.0 / t18 / t122;
        let t129 = 7.0 / 11664.0 * t32 * t111 * t56 + 7.0 / 23328.0 * t81 * t115 * t117 - 7.0 / 5832.0 * t81 * sigma[ip] * t35 * t124 * t99;
        let t133 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t129);
        let tvsigma0 = 2.0 * rho[ip] * t133;
        vsigma[ip] += tvsigma0;
    }
}
