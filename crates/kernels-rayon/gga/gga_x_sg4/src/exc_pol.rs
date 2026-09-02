//! GGA_X_SG4 exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_sg4.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_sg4_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
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
        let t29 = M_PI * M_PI;
        let t30 = pow_1_3(t29);
        let t31 = t30 * t30;
        let t32 = 1.0 / t31;
        let t33 = t28 * t32;
        let t34 = rho0 * rho0;
        let t35 = pow_1_3(rho0);
        let t36 = t35 * t35;
        let t38 = 1.0 / t36 / t34;
        let t40 = t33 * sigma0 * t38;
        let t42 = 1.0 - 0.0031233982573039467 * t40;
        let t43 = t28 * t28;
        let t44 = t29 * t29;
        let t45 = t44 * t29;
        let t47 = 1.0 / t30 / t45;
        let t48 = t43 * t47;
        let t49 = sigma0 * sigma0;
        let t50 = t49 * t49;
        let t51 = t50 * sigma0;
        let t52 = t34 * t34;
        let t53 = t52 * rho0;
        let t54 = t52 * t52;
        let t55 = t54 * t53;
        let t57 = 1.0 / t35 / t55;
        let t61 = 1.0 - 1.7835614159590037e-12 * t48 * t51 * t57;
        let t62 = 1.0 / t61;
        let t66 = 1.0 + 0.03727064220183486 * t40;
        let t69 = 1.804 - 0.5602871794871794 * t42 * t62 - 0.2437128205128205 / t66;
        let t73 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t69);
        let t74 = rho1 <= dens_threshold;
        let t75 = -t16;
        let t77 = piecewise5(t14, t11, t10, t15, t75 * t7);
        let t78 = 1.0 + t77;
        let t79 = t78 <= zeta_threshold;
        let t80 = pow_1_3(t78);
        let t82 = piecewise3(t79, t22, t80 * t78);
        let t83 = t82 * t26;
        let t84 = rho1 * rho1;
        let t85 = pow_1_3(rho1);
        let t86 = t85 * t85;
        let t88 = 1.0 / t86 / t84;
        let t90 = t33 * sigma2 * t88;
        let t92 = 1.0 - 0.0031233982573039467 * t90;
        let t93 = sigma2 * sigma2;
        let t94 = t93 * t93;
        let t95 = t94 * sigma2;
        let t96 = t84 * t84;
        let t97 = t96 * rho1;
        let t98 = t96 * t96;
        let t99 = t98 * t97;
        let t101 = 1.0 / t85 / t99;
        let t105 = 1.0 - 1.7835614159590037e-12 * t48 * t95 * t101;
        let t106 = 1.0 / t105;
        let t110 = 1.0 + 0.03727064220183486 * t90;
        let t113 = 1.804 - 0.5602871794871794 * t92 * t106 - 0.2437128205128205 / t110;
        let t117 = piecewise3(t74, 0.0, -3.0 / 8.0 * t5 * t83 * t113);
        let tzk0 = t73 + t117;
        zk[ip] += tzk0;
    }
}
