//! GGA_X_ITYH exc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_ityh.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_2};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_ityh_exc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t7 = 1.0 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = piecewise5::<f64>(t7, t8, t7, -t8, 0.0);
        let t11 = 1.0 + t10;
        let t13 = pow_1_3::<f64>(zeta_threshold);
        let t15 = pow_1_3::<f64>(t11);
        let t17 = piecewise3::<f64>(t11 <= zeta_threshold, t13 * zeta_threshold, t15 * t11);
        let t18 = t3 / t4 * t17;
        let t19 = pow_1_3::<f64>(rho[ip]);
        let t20 = t3 * t3;
        let t22 = 1.0 / M_PI;
        let t23 = pow_1_3::<f64>(t22);
        let t24 = 1.0 / t23;
        let t25 = M_CBRT4;
        let t26 = t24 * t25;
        let t27 = t20 * t24;
        let t28 = t27 * t25;
        let t29 = M_CBRT2;
        let t30 = t29 * t29;
        let t31 = sigma[ip] * t30;
        let t32 = rho[ip] * rho[ip];
        let t33 = t19 * t19;
        let t35 = 1.0 / t33 / t32;
        let t36 = f64::sqrt(sigma[ip]);
        let t37 = t36 * t29;
        let t39 = 1.0 / t19 / rho[ip];
        let t41 = f64::ln(t37 * t39 + f64::sqrt(pow_2::<f64>(t37 * t39) + 1.0));
        let t42 = t39 * t41;
        let t45 = 1.0 + 0.252e-1 * t37 * t42;
        let t46 = 1.0 / t45;
        let t51 = 1.0 + 0.93333333333333333332e-3 * t28 * t31 * t35 * t46;
        let t54 = M_PI * t20 * t26 / t51;
        let t55 = f64::sqrt(t54);
        let t57 = param_hyb_omega_0 / t55;
        let t58 = t11 * rho[ip];
        let t59 = pow_1_3::<f64>(t58);
        let t60 = 1.0 / t59;
        let t61 = t29 * t60;
        let t63 = t57 * t61 / 2.0;
        let t64 = 0.135e1 <= t63;
        let t65 = 0.135e1 < t63;
        let t66 = piecewise3::<f64>(t65, t63, 0.135e1);
        let t67 = t66 * t66;
        let t70 = t67 * t67;
        let t71 = 1.0 / t70;
        let t73 = t70 * t67;
        let t74 = 1.0 / t73;
        let t76 = t70 * t70;
        let t77 = 1.0 / t76;
        let t80 = 1.0 / t76 / t67;
        let t83 = 1.0 / t76 / t70;
        let t86 = 1.0 / t76 / t73;
        let t88 = t76 * t76;
        let t89 = 1.0 / t88;
        let t92 = piecewise3::<f64>(t65, 0.135e1, t63);
        let t93 = f64::sqrt(M_PI);
        let t94 = 1.0 / t92;
        let t96 = erf_approx::<f64>(t94 / 2.0);
        let t98 = t92 * t92;
        let t99 = 1.0 / t98;
        let t101 = f64::exp(-t99 / 4.0);
        let t102 = t101 - 1.0;
        let t105 = t101 - 3.0 / 2.0 - 2.0 * t98 * t102;
        let t108 = 2.0 * t92 * t105 + t93 * t96;
        let t112 = piecewise3::<f64>(t64, 1.0 / t67 / 36.0 - t71 / 960.0 + t74 / 26880.0 - t77 / 829440.0 + t80 / 28385280.0 - t83 / 0.107347968e10 + t86 / 0.445906944e11 - t89 / 0.20214448128e13, 1.0 - 8.0 / 3.0 * t92 * t108);
        let t113 = t19 * t112;
        let t117 = piecewise3::<f64>(t2, 0.0, -3.0 / 8.0 * t18 * t113 * t51);
        let tzk0 = 2.0 * t117;
        zk[ip] += tzk0;
    }
}
