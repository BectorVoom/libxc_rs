//! GGA_X_BEEFVDW exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_beefvdw.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_beefvdw_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
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
        let t81 = -5427.777462637186 * t49 + 4135.586188014654 * t48 * t52 - 29150.193011493262 * t57 + 40074.93585443239 * t60 + 90365.6111085228 * t63 - 161142.1539984628 * t66 - 132044.6618218215 * t68 + 255894.79526235335 * t71 - 0.6945973517763898 * t45 + 0.527556201155898 * t55 - 0.38916037779196816 * t44 + 86.00573049927964 * t65 + 30.54203495931585 * t70 + 279670.48856303055 * t78 + 0.037534251004296526 * t41;
        let t88 = t46 * t70;
        let t91 = t48 * t45;
        let t93 = t48 * t51;
        let t95 = t48 * t55;
        let t97 = t48 * t43;
        let t99 = t46 * t65;
        let t102 = 1.1313514630621233 - 7.2975787893717134 * t51 + 3783.53964072524 * t59 - 617.547861045286 * t62 - 442.33229018433804 * t46 - 20148.24517562505 * t47 + 2274.8997850816486 * t56 + 70504.54186903402 * t88 - 2810.240180568463 * t52 - 323524.0313604933 * t91 + 180782.00670879145 * t93 - 129814.81812794984 * t95 + 56174.00797937267 * t97 - 10276.426607863825 * t99 - 168370.8413901412 * t48;
        let t103 = t81 + t102;
        let t107 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t103);
        let tzk0 = 2.0 * t107;
        zk[ip] += tzk0;
    }
}
