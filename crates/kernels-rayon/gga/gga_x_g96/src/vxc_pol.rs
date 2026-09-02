//! GGA_X_G96 vxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_g96.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_g96_vxc_pol(
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
        let t1 = rho0 <= dens_threshold;
        let t2 = M_CBRT3;
        let t3 = M_CBRTPI;
        let t4 = 1.0 / t3;
        let t5 = t2 * t4;
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
        let t28 = t2 * t2;
        let t30 = pow_1_3(1.0 / M_PI);
        let t31 = 1.0 / t30;
        let t32 = t28 * t31;
        let t33 = M_CBRT4;
        let t34 = rmath::sqrt(sigma0);
        let t35 = pow_1_3(rho0);
        let t37 = 1.0 / t35 / rho0;
        let t38 = t34 * t37;
        let t39 = rmath::sqrt(t38);
        let t40 = t39 * t38;
        let t44 = 1.0 + 2.0 / 1233.0 * t32 * t33 * t40;
        let t48 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t25 * t26 * t44);
        let t49 = rho1 <= dens_threshold;
        let t50 = -t16;
        let t52 = piecewise5(t14, t11, t10, t15, t50 * t7);
        let t53 = 1.0 + t52;
        let t54 = t53 <= zeta_threshold;
        let t55 = pow_1_3(t53);
        let t57 = piecewise3(t54, t22, t55 * t53);
        let t59 = rmath::sqrt(sigma2);
        let t60 = pow_1_3(rho1);
        let t62 = 1.0 / t60 / rho1;
        let t63 = t59 * t62;
        let t64 = rmath::sqrt(t63);
        let t65 = t64 * t63;
        let t69 = 1.0 + 2.0 / 1233.0 * t32 * t33 * t65;
        let t73 = piecewise3(t49, 0.0, -3.0 / 8.0 * t5 * t57 * t26 * t69);
        let tzk0 = t48 + t73;
        zk[ip] += tzk0;
        let t74 = t6 * t6;
        let t75 = 1.0 / t74;
        let t76 = t16 * t75;
        let t78 = piecewise5(t10, 0.0, t14, 0.0, t7 - t76);
        let t81 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t78);
        let t86 = t26 * t26;
        let t87 = 1.0 / t86;
        let t91 = t5 * t25 * t87 * t44 / 8.0;
        let t92 = t4 * t25;
        let t93 = t26 * t31;
        let t94 = t92 * t93;
        let t95 = t33 * t39;
        let t96 = rho0 * rho0;
        let t98 = 1.0 / t35 / t96;
        let t100 = t95 * t34 * t98;
        let t104 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t81 * t26 * t44 - t91 + t94 * t100 / 274.0);
        let t105 = t50 * t75;
        let t107 = piecewise5(t14, 0.0, t10, 0.0, -t7 - t105);
        let t110 = piecewise3(t54, 0.0, 4.0 / 3.0 * t55 * t107);
        let t118 = t5 * t57 * t87 * t69 / 8.0;
        let t120 = piecewise3(t49, 0.0, -3.0 / 8.0 * t5 * t110 * t26 * t69 - t118);
        let tvrho0 = t48 + t73 + t6 * (t104 + t120);
        vrho[ip * 2] += tvrho0;
        let t124 = piecewise5(t10, 0.0, t14, 0.0, -t7 - t76);
        let t127 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t124);
        let t133 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t127 * t26 * t44 - t91);
        let t135 = piecewise5(t14, 0.0, t10, 0.0, t7 - t105);
        let t138 = piecewise3(t54, 0.0, 4.0 / 3.0 * t55 * t135);
        let t143 = t4 * t57;
        let t144 = t143 * t93;
        let t145 = t33 * t64;
        let t146 = rho1 * rho1;
        let t148 = 1.0 / t60 / t146;
        let t150 = t145 * t59 * t148;
        let t154 = piecewise3(t49, 0.0, -3.0 / 8.0 * t5 * t138 * t26 * t69 - t118 + t144 * t150 / 274.0);
        let tvrho1 = t48 + t73 + t6 * (t133 + t154);
        vrho[ip * 2 + 1] += tvrho1;
        let t157 = 1.0 / t34;
        let t159 = t95 * t157 * t37;
        let t162 = piecewise3(t1, 0.0, -3.0 / 2192.0 * t94 * t159);
        let tvsigma0 = t6 * t162;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t163 = 1.0 / t59;
        let t165 = t145 * t163 * t62;
        let t168 = piecewise3(t49, 0.0, -3.0 / 2192.0 * t144 * t165);
        let tvsigma2 = t6 * t168;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
