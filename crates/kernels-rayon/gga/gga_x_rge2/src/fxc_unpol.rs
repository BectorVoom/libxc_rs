//! GGA_X_RGE2 fxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_rge2.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_rge2_fxc_unpol(
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
        let t20 = M_CBRT6;
        let t21 = M_PI * M_PI;
        let t22 = pow_1_3(t21);
        let t23 = t22 * t22;
        let t25 = t20 / t23;
        let t26 = M_CBRT2;
        let t27 = t26 * t26;
        let t28 = sigma[ip] * t27;
        let t29 = rho[ip] * rho[ip];
        let t30 = t18 * t18;
        let t32 = 1.0 / t30 / t29;
        let t36 = t20 * t20;
        let t38 = 1.0 / t22 / t21;
        let t39 = t36 * t38;
        let t40 = sigma[ip] * sigma[ip];
        let t41 = t40 * t26;
        let t42 = t29 * t29;
        let t43 = t42 * rho[ip];
        let t45 = 1.0 / t18 / t43;
        let t49 = 0.804 + 5.0 / 972.0 * t25 * t28 * t32 + 6.582356890714508e-05 * t39 * t41 * t45;
        let t52 = 1.804 - 0.646416 / t49;
        let t56 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t17 * t18 * t52);
        let tzk0 = 2.0 * t56;
        zk[ip] += tzk0;
        let t57 = 1.0 / t30;
        let t62 = t3 * t17;
        let t63 = t49 * t49;
        let t64 = 1.0 / t63;
        let t65 = t18 * t64;
        let t66 = t29 * rho[ip];
        let t68 = 1.0 / t30 / t66;
        let t72 = t42 * t29;
        let t74 = 1.0 / t18 / t72;
        let t78 = -10.0 / 729.0 * t25 * t28 * t68 - 0.00035105903417144045 * t39 * t41 * t74;
        let t83 = piecewise3(t2, 0.0, -t6 * t17 * t57 * t52 / 8.0 - 0.1655109536374632 * t62 * t65 * t78);
        let tvrho0 = 2.0 * rho[ip] * t83 + 2.0 * t56;
        vrho[ip] += tvrho0;
        let t89 = sigma[ip] * t26;
        let t93 = 5.0 / 972.0 * t25 * t27 * t32 + 0.00013164713781429015 * t39 * t89 * t45;
        let t97 = piecewise3(t2, 0.0, -0.1655109536374632 * t62 * t65 * t93);
        let tvsigma0 = 2.0 * rho[ip] * t97;
        vsigma[ip] += tvsigma0;
        let t101 = 1.0 / t30 / rho[ip];
        let t106 = t57 * t64;
        let t111 = 1.0 / t63 / t49;
        let t112 = t18 * t111;
        let t113 = t78 * t78;
        let t118 = 1.0 / t30 / t42;
        let t122 = t42 * t66;
        let t124 = 1.0 / t18 / t122;
        let t128 = 110.0 / 2187.0 * t25 * t28 * t118 + 0.0022233738830857892 * t39 * t41 * t124;
        let t133 = piecewise3(t2, 0.0, t6 * t17 * t101 * t52 / 12.0 - 0.1103406357583088 * t62 * t106 * t78 + 0.3310219072749264 * t62 * t112 * t113 - 0.1655109536374632 * t62 * t65 * t128);
        let tv2rho20 = 2.0 * rho[ip] * t133 + 4.0 * t83;
        v2rho2[ip] += tv2rho20;
        let t139 = t62 * t18;
        let t140 = t111 * t93;
        let t141 = t140 * t78;
        let t150 = -10.0 / 729.0 * t25 * t27 * t68 - 0.0007021180683428809 * t39 * t89 * t74;
        let t155 = piecewise3(t2, 0.0, -0.0551703178791544 * t62 * t106 * t93 + 0.3310219072749264 * t139 * t141 - 0.1655109536374632 * t62 * t65 * t150);
        let tv2rhosigma0 = 2.0 * rho[ip] * t155 + 2.0 * t97;
        v2rhosigma[ip] += tv2rhosigma0;
        let t158 = t93 * t93;
        let t162 = 1.0 / t43;
        let t165 = t38 * t26;
        let t166 = t64 * t36 * t165;
        let t170 = piecewise3(t2, 0.0, 0.3310219072749264 * t62 * t112 * t158 - 2.1789043323285708e-05 * t62 * t162 * t166);
        let tv2sigma20 = 2.0 * rho[ip] * t170;
        v2sigma2[ip] += tv2sigma20;
    }
}
