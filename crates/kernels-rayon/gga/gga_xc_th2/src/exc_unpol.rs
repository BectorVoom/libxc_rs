//! GGA_XC_TH2 exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_xc_th2.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_xc_th2_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t1 = rmath::pow(2.0, 1.0 / 12.0);
        let t2 = t1 * t1;
        let t3 = t2 * t1;
        let t4 = t2 * t2;
        let t5 = t4 * t4;
        let t6 = t5 * t3;
        let t7 = rmath::pow(rho[ip], 1.0 / 12.0);
        let t11 = rmath::pow(2.0, 1.0 / 6.0);
        let t12 = t11 * t11;
        let t13 = t12 * t12;
        let t14 = t13 * t11;
        let t15 = rmath::pow(rho[ip], 1.0 / 6.0);
        let t16 = t15 * rho[ip];
        let t19 = M_CBRT2;
        let t20 = t19 * t19;
        let t21 = pow_1_3(rho[ip]);
        let t22 = t21 * rho[ip];
        let t25 = M_SQRT2;
        let t26 = rmath::sqrt(rho[ip]);
        let t27 = t26 * rho[ip];
        let t30 = t21 * t21;
        let t31 = t30 * rho[ip];
        let t32 = t19 * t31;
        let t34 = t4 * t3;
        let t35 = t34 * t7;
        let t36 = rmath::sqrt(sigma[ip]);
        let t38 = pow_1_3(zeta_threshold);
        let t40 = piecewise3(1.0 <= zeta_threshold, t38 * zeta_threshold, 1.0);
        let t41 = t36 * t40;
        let t44 = t25 * t15;
        let t47 = t19 * t21;
        let t50 = t11 * t26;
        let t53 = 1.0 / rho[ip];
        let t54 = t19 * t53;
        let t55 = t40 * t40;
        let t56 = sigma[ip] * t55;
        let t59 = t15 * t15;
        let t60 = t59 * t59;
        let t61 = t60 * t15;
        let t62 = 1.0 / t61;
        let t63 = t11 * t62;
        let t66 = 1.0 / t30;
        let t70 = rho[ip] * rho[ip];
        let t72 = 1.0 / t30 / t70;
        let t73 = sigma[ip] * t72;
        let t74 = t73 * t55;
        let t75 = t74 - t73;
        let t78 = t61 * rho[ip];
        let t79 = t11 * t78;
        let t84 = 0.3394155 * t6 * t7 * rho[ip] - 0.879105 * t14 * t16 + 0.63838 * t20 * t22 - 0.803945 * t25 * t27 + 0.182805 * t32 - 0.04533175 * t35 * t41 + 0.03674325 * t44 * t41 + 0.03678525 * t47 * t41 - 0.017922925 * t50 * t41 - 0.0050895875 * t54 * t56 + 0.0026828125 * t63 * t56 - 9.60195e-05 * t66 * sigma[ip] * t55 + 0.01551885 * t32 * t75 - 0.0360163 * t79 * t75 + 0.0223281 * t70 * t75;
        let tzk0 = t84 * t53;
        zk[ip] += tzk0;
    }
}
