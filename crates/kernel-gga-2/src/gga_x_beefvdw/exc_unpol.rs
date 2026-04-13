//! GGA_X_BEEFVDW exc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_beefvdw.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_beefvdw_exc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t7 = 1.0 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = piecewise5(t7, t8, t7, -t8, 0.0);
        let t11 = t10 + 1.0;
        let t13 = pow_1_3(zeta_threshold);
        let t15 = pow_1_3(t11);
        let t17 = piecewise3(t11 <= zeta_threshold, t13 * zeta_threshold, t15 * t11);
        let t18 = pow_1_3(rho[ip]);
        let t19 = t17 * t18;
        let t20 = M_CBRT6;
        let t21 = M_PI * M_PI;
        let t22 = pow_1_3(t21);
        let t23 = t22 * t22;
        let t24 = 1.0 / t23;
        let t25 = t20 * t24;
        let t26 = t25 * sigma[ip];
        let t27 = M_CBRT2;
        let t28 = t27 * t27;
        let t29 = rho[ip] * rho[ip];
        let t30 = t18 * t18;
        let t32 = 1.0 / t30 / t29;
        let t38 = 4.0 + t25 * sigma[ip] * t28 * t32 / 24.0;
        let t39 = 1.0 / t38;
        let t40 = t28 * t32 * t39;
        let t41 = t26 * t40;
        let t43 = t41 / 12.0 - 1.0;
        let t44 = t43 * t43;
        let t45 = t44 * t44;
        let t46 = t45 * t45;
        let t47 = t46 * t45;
        let t48 = t46 * t46;
        let t49 = t48 * t47;
        let t51 = t45 * t43;
        let t52 = t46 * t51;
        let t55 = t44 * t43;
        let t56 = t46 * t55;
        let t57 = t48 * t56;
        let t59 = t46 * t44;
        let t60 = t48 * t59;
        let t62 = t46 * t43;
        let t63 = t48 * t62;
        let t65 = t45 * t55;
        let t66 = t48 * t65;
        let t68 = t48 * t46;
        let t70 = t45 * t44;
        let t71 = t48 * t70;
        let t78 = t48 * t44;
        let t81 = -0.5427777462637186032e4 * t49 + 0.4135586188014653875e4 * t48 * t52 - 0.29150193011493262292e5 * t57 + 0.40074935854432390114e5 * t60 + 0.90365611108522808258e5 * t63 - 0.16114215399846280595e6 * t66 - 0.13204466182182150467e6 * t68 + 0.2558947952623533461e6 * t71 - 0.69459735177638985466e0 * t45 + 0.52755620115589800943e0 * t55 - 0.38916037779196815969e0 * t44 + 0.86005730499279641299e2 * t65 + 0.30542034959315850168e2 * t70 + 0.27967048856303053872e6 * t78 + 0.37534251004296526981e-1 * t41;
        let t88 = t46 * t70;
        let t91 = t48 * t45;
        let t93 = t48 * t51;
        let t95 = t48 * t55;
        let t97 = t48 * t43;
        let t99 = t46 * t65;
        let t102 = 0.11313514630621233134e1 - 0.72975787893717136018e1 * t51 + 0.37835396407252402359e4 * t59 - 0.61754786104528599731e3 * t62 - 0.44233229018433803622e3 * t46 - 0.20148245175625047025e5 * t47 + 0.22748997850816485208e4 * t56 + 0.70504541869034010051e5 * t88 - 0.281024018056846299e4 * t52 - 0.32352403136049329184e6 * t91 + 0.18078200670879145336e6 * t93 - 0.12981481812794983922e6 * t95 + 0.56174007979372666951e5 * t97 - 0.10276426607863824397e5 * t99 - 0.16837084139014120539e6 * t48;
        let t103 = t81 + t102;
        let t107 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t103);
        let tzk0 = 2.0 * t107;
        zk[ip] += tzk0;
    }
}
