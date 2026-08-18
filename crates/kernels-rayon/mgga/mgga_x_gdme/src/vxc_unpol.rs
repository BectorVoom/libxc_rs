//! MGGA_X_GDME vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_gdme.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_gdme_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    param_a: f64,
    param_AA: f64,
    param_BB: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = M_CBRTPI;
        let t6 = 1.0 / t5;
        let t7 = t4 * t6;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t16 = pow_1_3(t12);
        let t18 = piecewise3(t12 <= zeta_threshold, t14 * zeta_threshold, t16 * t12);
        let t19 = pow_1_3(rho[ip]);
        let t20 = t18 * t19;
        let t23 = M_CBRT2;
        let t26 = pow_1_3(1.0 / M_PI);
        let t27 = 1.0 / t26;
        let t28 = M_CBRT4;
        let t29 = t27 * t28;
        let t30 = M_PI * M_PI;
        let t31 = pow_1_3(t30);
        let t32 = t31 * t31;
        let t39 = t23 * t23;
        let t42 = 1.0 / t31 / t30;
        let t43 = param_a * param_a;
        let t44 = t43 - param_a + 1.0 / 2.0;
        let t45 = t44 * lapl[ip];
        let t46 = t19 * t19;
        let t48 = 1.0 / t46 / rho[ip];
        let t51 = tau[ip] * t39;
        let t59 = 2.0 / 9.0 * (param_AA + 3.0 / 5.0 * param_BB) * t23 * t29 / t32 + param_BB * t4 * t27 * t28 * t39 * t42 * (t45 * t39 * t48 - 2.0 * t51 * t48) / 27.0;
        let t63 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t59);
        let tzk0 = 2.0 * t63;
        zk[ip] += tzk0;
        let t65 = t18 / t46;
        let t69 = t4 * t4;
        let t70 = t69 * t6;
        let t72 = t70 * t20 * param_BB;
        let t73 = t39 * t42;
        let t74 = rho[ip] * rho[ip];
        let t76 = 1.0 / t46 / t74;
        let t84 = t29 * t73 * (-5.0 / 3.0 * t45 * t39 * t76 + 10.0 / 3.0 * t51 * t76);
        let t88 = piecewise3(t3, 0.0, -t7 * t65 * t59 / 8.0 - t72 * t84 / 72.0);
        let tvrho0 = 2.0 * rho[ip] * t88 + 2.0 * t63;
        vrho[ip] += tvrho0;
        let tvsigma0 = 0.0;
        vsigma[ip] += tvsigma0;
        let t93 = t18 / t19 / rho[ip];
        let t98 = t29 * t23 * t42 * t44;
        let t101 = piecewise3(t3, 0.0, -t70 * t93 * param_BB * t98 / 36.0);
        let tvlapl0 = 2.0 * rho[ip] * t101;
        vlapl[ip] += tvlapl0;
        let t107 = param_BB * t27 * t28 * t23 * t42;
        let t110 = piecewise3(t3, 0.0, t70 * t93 * t107 / 18.0);
        let tvtau0 = 2.0 * rho[ip] * t110;
        vtau[ip] += tvtau0;
    }
}
