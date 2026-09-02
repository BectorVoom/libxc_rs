//! GGA_C_WL vxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_wl.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_wl_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let t1 = rho0 - rho1;
        let t2 = t1 * t1;
        let t3 = rho0 + rho1;
        let t4 = t3 * t3;
        let t5 = 1.0 / t4;
        let t7 = -t2 * t5 + 1.0;
        let t8 = rmath::sqrt(t7);
        let t10 = sigma0 + 2.0 * sigma1 + sigma2;
        let t11 = rmath::sqrt(t10);
        let t12 = pow_1_3(t3);
        let t14 = 1.0 / t12 / t3;
        let t17 = -0.7486 + 0.06001 * t11 * t14;
        let t18 = t8 * t17;
        let t19 = rmath::sqrt(sigma0);
        let t20 = pow_1_3(rho0);
        let t22 = 1.0 / t20 / rho0;
        let t25 = rmath::sqrt(sigma2);
        let t26 = pow_1_3(rho1);
        let t28 = 1.0 / t26 / rho1;
        let t31 = M_CBRT3;
        let t33 = pow_1_3(1.0 / M_PI);
        let t34 = t31 * t33;
        let t35 = M_CBRT4;
        let t36 = t35 * t35;
        let t37 = 1.0 / t12;
        let t41 = 3.60073 + 0.9 * t19 * t22 + 0.9 * t25 * t28 + t34 * t36 * t37 / 4.0;
        let t42 = 1.0 / t41;
        let tzk0 = t18 * t42;
        zk[ip] += tzk0;
        let t43 = 1.0 / t8;
        let t44 = t3 * t43;
        let t45 = t17 * t42;
        let t46 = t1 * t5;
        let t47 = t4 * t3;
        let t48 = 1.0 / t47;
        let t49 = t2 * t48;
        let t51 = -2.0 * t46 + 2.0 * t49;
        let t55 = t14 * t8;
        let t56 = t11 * t42;
        let t58 = 0.08001333333333334 * t55 * t56;
        let t59 = t3 * t8;
        let t60 = t41 * t41;
        let t61 = 1.0 / t60;
        let t62 = t17 * t61;
        let t63 = rho0 * rho0;
        let t65 = 1.0 / t20 / t63;
        let t70 = t34 * t36 * t14 / 12.0;
        let t71 = -1.2 * t19 * t65 - t70;
        let tvrho0 = tzk0 + t44 * t45 * t51 / 2.0 - t58 - t59 * t62 * t71;
        vrho[ip * 2] += tvrho0;
        let t75 = 2.0 * t46 + 2.0 * t49;
        let t79 = rho1 * rho1;
        let t81 = 1.0 / t26 / t79;
        let t84 = -1.2 * t25 * t81 - t70;
        let tvrho1 = tzk0 + t44 * t45 * t75 / 2.0 - t58 - t59 * t62 * t84;
        vrho[ip * 2 + 1] += tvrho1;
        let t87 = t37 * t8;
        let t88 = 1.0 / t11;
        let t89 = t88 * t42;
        let t90 = t87 * t89;
        let t91 = 0.030005 * t90;
        let t92 = t59 * t17;
        let t93 = 1.0 / t19;
        let t94 = t61 * t93;
        let t95 = t94 * t22;
        let tvsigma0 = t91 - 0.45 * t92 * t95;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.06001 * t90;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t98 = 1.0 / t25;
        let t99 = t61 * t98;
        let t100 = t99 * t28;
        let tvsigma2 = t91 - 0.45 * t92 * t100;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
