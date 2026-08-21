//! GGA_X_2D_B86_MGC fxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_2d_b86_mgc.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_4};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_2d_b86_mgc_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = rmath::sqrt(M_PI);
        let t5 = 1.0 <= zeta_threshold;
        let t6 = zeta_threshold - 1.0;
        let t8 = piecewise5(t5, t6, t5, -t6, 0.0);
        let t9 = 1.0 + t8;
        let t11 = rmath::sqrt(zeta_threshold);
        let t13 = rmath::sqrt(t9);
        let t15 = piecewise3(t9 <= zeta_threshold, t11 * zeta_threshold, t13 * t9);
        let t16 = 1.0 / t3 * t15;
        let t17 = M_SQRT2;
        let t18 = rmath::sqrt(rho[ip]);
        let t19 = t17 * t18;
        let t20 = rho[ip] * rho[ip];
        let t21 = t20 * rho[ip];
        let t22 = 1.0 / t21;
        let t23 = sigma[ip] * t22;
        let t25 = 1.0 + 0.016646 * t23;
        let t26 = pow_1_4(t25);
        let t27 = t26 * t26;
        let t28 = t27 * t26;
        let t29 = 1.0 / t28;
        let t32 = 1.0 + 0.004409422067590198 * t23 * t29;
        let t36 = piecewise3(t2, 0.0, -2.0 / 3.0 * t16 * t19 * t32);
        let tzk0 = 2.0 * t36;
        zk[ip] += tzk0;
        let t38 = t17 / t18;
        let t42 = t20 * t20;
        let t43 = 1.0 / t42;
        let t47 = sigma[ip] * sigma[ip];
        let t48 = t42 * t21;
        let t49 = 1.0 / t48;
        let t52 = 1.0 / t28 / t25;
        let t55 = -0.013228266202770593 * sigma[ip] * t43 * t29 + 0.00016514828940848947 * t47 * t49 * t52;
        let t60 = piecewise3(t2, 0.0, -t16 * t38 * t32 / 3.0 - 2.0 / 3.0 * t16 * t19 * t55);
        let tvrho0 = 2.0 * rho[ip] * t60 + 2.0 * t36;
        vrho[ip] += tvrho0;
        let t65 = t42 * t20;
        let t66 = 1.0 / t65;
        let t67 = sigma[ip] * t66;
        let t70 = 0.004409422067590198 * t22 * t29 - 5.504942980282982e-05 * t67 * t52;
        let t74 = piecewise3(t2, 0.0, -2.0 / 3.0 * t16 * t19 * t70);
        let tvsigma0 = 2.0 * rho[ip] * t74;
        vsigma[ip] += tvsigma0;
        let t79 = t17 / t18 / rho[ip];
        let t86 = t42 * rho[ip];
        let t87 = 1.0 / t86;
        let t91 = t42 * t42;
        let t92 = 1.0 / t91;
        let t96 = t47 * sigma[ip];
        let t98 = 1.0 / t91 / t21;
        let t100 = t25 * t25;
        let t102 = 1.0 / t28 / t100;
        let t105 = 0.05291306481108237 * sigma[ip] * t87 * t29 - 0.0016514828940848946 * t47 * t92 * t52 + 1.4432556733842006e-05 * t96 * t98 * t102;
        let t110 = piecewise3(t2, 0.0, t16 * t79 * t32 / 6.0 - 2.0 / 3.0 * t16 * t38 * t55 - 2.0 / 3.0 * t16 * t19 * t105);
        let tv2rho20 = 2.0 * rho[ip] * t110 + 4.0 * t60;
        v2rho2[ip] += tv2rho20;
        let t118 = t49 * t52;
        let t122 = 1.0 / t91 / t20;
        let t123 = t47 * t122;
        let t126 = -0.013228266202770593 * t43 * t29 + 0.0004954448682254683 * t118 * sigma[ip] - 4.810852244614002e-06 * t123 * t102;
        let t131 = piecewise3(t2, 0.0, -t16 * t38 * t70 / 3.0 - 2.0 / 3.0 * t16 * t19 * t126);
        let tv2rhosigma0 = 2.0 * rho[ip] * t131 + 2.0 * t74;
        v2rhosigma[ip] += tv2rhosigma0;
        let t137 = 1.0 / t91 / rho[ip];
        let t141 = -0.00011009885960565965 * t66 * t52 + 1.6036174148713342e-06 * sigma[ip] * t137 * t102;
        let t145 = piecewise3(t2, 0.0, -2.0 / 3.0 * t16 * t19 * t141);
        let tv2sigma20 = 2.0 * rho[ip] * t145;
        v2sigma2[ip] += tv2sigma20;
    }
}
