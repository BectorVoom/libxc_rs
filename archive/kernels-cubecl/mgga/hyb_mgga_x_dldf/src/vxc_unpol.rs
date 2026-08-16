//! HYB_MGGA_X_DLDF vxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/hyb_mgga_x_dldf.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn hyb_mgga_x_dldf_vxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    vlapl: &mut Array<f64>,
    vtau: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = 1.0 <= zeta_threshold;
        let t6 = zeta_threshold - 1.0;
        let t8 = piecewise5::<f64>(t5, t6, t5, -t6, 0.0);
        let t9 = 1.0 + t8;
        let t11 = pow_1_3::<f64>(zeta_threshold);
        let t13 = pow_1_3::<f64>(t9);
        let t15 = piecewise3::<f64>(t9 <= zeta_threshold, t11 * zeta_threshold, t13 * t9);
        let t16 = t4 * t15;
        let t17 = pow_1_3::<f64>(rho[ip]);
        let t18 = M_CBRT6;
        let t19 = M_PI * M_PI;
        let t20 = pow_1_3::<f64>(t19);
        let t21 = t20 * t20;
        let t23 = t18 / t21;
        let t24 = M_CBRT2;
        let t25 = t24 * t24;
        let t26 = sigma[ip] * t25;
        let t27 = rho[ip] * rho[ip];
        let t28 = t17 * t17;
        let t30 = 1.0 / t28 / t27;
        let t34 = 0.48827323e1 + 0.146297e-1 * t23 * t26 * t30;
        let t37 = 0.58827323e1 - 0.2384107471346329e2 / t34;
        let t38 = t17 * t37;
        let t39 = t18 * t18;
        let t41 = 3.0 / 10.0 * t39 * t21;
        let t42 = tau[ip] * t25;
        let t44 = 1.0 / t28 / rho[ip];
        let t45 = t42 * t44;
        let t46 = t41 - t45;
        let t47 = t41 + t45;
        let t48 = 1.0 / t47;
        let t51 = t46 * t46;
        let t52 = t47 * t47;
        let t53 = 1.0 / t52;
        let t56 = t51 * t46;
        let t57 = t52 * t47;
        let t58 = 1.0 / t57;
        let t61 = t51 * t51;
        let t62 = t52 * t52;
        let t63 = 1.0 / t62;
        let t66 = 1.0 - 0.1637571e0 * t46 * t48 - 0.1880028e0 * t51 * t53 - 0.4490609e0 * t56 * t58 - 0.82359e-2 * t61 * t63;
        let t70 = piecewise3::<f64>(t3, 0.0, -0.98727272578809758046e-1 * t16 * t38 * t66);
        let tzk0 = 2.0 * t70;
        zk[ip] += tzk0;
        let t72 = 1.0 / t28 * t37;
        let t76 = t27 * rho[ip];
        let t78 = 1.0 / t17 / t76;
        let t79 = t34 * t34;
        let t80 = 1.0 / t79;
        let t82 = t16 * t78 * t80;
        let t84 = t23 * t26 * t66;
        let t90 = t46 * t53;
        let t91 = t42 * t30;
        let t94 = t51 * t58;
        let t97 = t56 * t63;
        let t101 = 1.0 / t62 / t47;
        let t102 = t61 * t101;
        let t105 = -0.2729285e0 * t42 * t30 * t48 - 0.8996045e0 * t90 * t91 - 0.28719805e1 * t94 * t91 - 0.23002105e1 * t97 * t91 - 0.54906e-1 * t102 * t91;
        let t110 = piecewise3::<f64>(t3, 0.0, -0.32909090859603252682e-1 * t16 * t72 * t66 + 0.91826307502838492063e-1 * t82 * t84 - 0.98727272578809758046e-1 * t16 * t38 * t105);
        let tvrho0 = 2.0 * rho[ip] * t110 + 2.0 * t70;
        vrho[ip] += tvrho0;
        let t116 = t16 / t17 / t27 * t80;
        let t118 = t23 * t25 * t66;
        let t121 = piecewise3::<f64>(t3, 0.0, -0.34434865313564434524e-1 * t116 * t118);
        let tvsigma0 = 2.0 * rho[ip] * t121;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t123 = t25 * t44;
        let t134 = 0.1637571e0 * t123 * t48 + 0.5397627e0 * t90 * t123 + 0.17231883e1 * t94 * t123 + 0.13801263e1 * t97 * t123 + 0.329436e-1 * t102 * t123;
        let t138 = piecewise3::<f64>(t3, 0.0, -0.98727272578809758046e-1 * t16 * t38 * t134);
        let tvtau0 = 2.0 * rho[ip] * t138;
        vtau[ip] += tvtau0;
    }
}
