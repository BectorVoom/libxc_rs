//! MGGA_X_TH vxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_th.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT4, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_th_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let lapl0 = lapl[ip * 2];
        let lapl1 = lapl[ip * 2 + 1];
        let tau0 = tau[ip * 2];
        let tau1 = tau[ip * 2 + 1];
        let t2 = rho0 <= dens_threshold;
        let t3 = M_CBRTPI;
        let t4 = t3 * t3;
        let t5 = rho0 + rho1;
        let t6 = 1.0 / t5;
        let t9 = 2.0 * rho0 * t6 <= zeta_threshold;
        let t10 = zeta_threshold - 1.0;
        let t13 = 2.0 * rho1 * t6 <= zeta_threshold;
        let t14 = -t10;
        let t15 = rho0 - rho1;
        let t17 = piecewise5(t9, t10, t13, t14, t15 * t6);
        let t18 = 1.0 + t17;
        let t19 = t18 <= zeta_threshold;
        let t20 = pow_1_3(zeta_threshold);
        let t21 = t20 * zeta_threshold;
        let t22 = pow_1_3(t18);
        let t24 = piecewise3(t19, t21, t22 * t18);
        let t25 = t4 * t24;
        let t26 = pow_1_3(t5);
        let t27 = 1.0 / tau0;
        let t28 = t26 * t27;
        let t29 = t25 * t28;
        let t30 = pow_1_3(rho0);
        let t31 = t30 * t30;
        let t37 = 1.0 + 7.0 / 216.0 * sigma0 / rho0 * t27;
        let t40 = pow_1_3(1.0 / M_PI);
        let t42 = M_CBRT4;
        let t43 = 1.0 / t40 * t42;
        let t44 = t31 * rho0 * t37 * t43;
        let t47 = piecewise3(t2, 0.0, -27.0 / 80.0 * t29 * t44);
        let t48 = rho1 <= dens_threshold;
        let t49 = -t15;
        let t51 = piecewise5(t13, t10, t9, t14, t49 * t6);
        let t52 = 1.0 + t51;
        let t53 = t52 <= zeta_threshold;
        let t54 = pow_1_3(t52);
        let t56 = piecewise3(t53, t21, t54 * t52);
        let t57 = t4 * t56;
        let t58 = 1.0 / tau1;
        let t59 = t26 * t58;
        let t60 = t57 * t59;
        let t61 = pow_1_3(rho1);
        let t62 = t61 * t61;
        let t68 = 1.0 + 7.0 / 216.0 * sigma2 / rho1 * t58;
        let t70 = t62 * rho1 * t68 * t43;
        let t73 = piecewise3(t48, 0.0, -27.0 / 80.0 * t60 * t70);
        let tzk0 = t47 + t73;
        zk[ip] += tzk0;
        let t74 = t5 * t5;
        let t75 = 1.0 / t74;
        let t76 = t15 * t75;
        let t78 = piecewise5(t9, 0.0, t13, 0.0, t6 - t76);
        let t81 = piecewise3(t19, 0.0, 4.0 / 3.0 * t22 * t78);
        let t82 = t4 * t81;
        let t83 = t82 * t28;
        let t86 = t26 * t26;
        let t87 = 1.0 / t86;
        let t88 = t87 * t27;
        let t89 = t25 * t88;
        let t91 = 9.0 / 80.0 * t89 * t44;
        let t93 = t31 * t37 * t43;
        let t96 = tau0 * tau0;
        let t97 = 1.0 / t96;
        let t98 = t26 * t97;
        let t99 = t25 * t98;
        let t100 = 1.0 / t30;
        let t102 = t100 * sigma0 * t43;
        let t106 = piecewise3(t2, 0.0, -27.0 / 80.0 * t83 * t44 - t91 - 9.0 / 16.0 * t29 * t93 + 7.0 / 640.0 * t99 * t102);
        let t107 = t49 * t75;
        let t109 = piecewise5(t13, 0.0, t9, 0.0, -t6 - t107);
        let t112 = piecewise3(t53, 0.0, 4.0 / 3.0 * t54 * t109);
        let t113 = t4 * t112;
        let t114 = t113 * t59;
        let t117 = t87 * t58;
        let t118 = t57 * t117;
        let t120 = 9.0 / 80.0 * t118 * t70;
        let t122 = piecewise3(t48, 0.0, -27.0 / 80.0 * t114 * t70 - t120);
        let tvrho0 = t47 + t73 + t5 * (t106 + t122);
        vrho[ip * 2] += tvrho0;
        let t126 = piecewise5(t9, 0.0, t13, 0.0, -t6 - t76);
        let t129 = piecewise3(t19, 0.0, 4.0 / 3.0 * t22 * t126);
        let t130 = t4 * t129;
        let t131 = t130 * t28;
        let t135 = piecewise3(t2, 0.0, -27.0 / 80.0 * t131 * t44 - t91);
        let t137 = piecewise5(t13, 0.0, t9, 0.0, t6 - t107);
        let t140 = piecewise3(t53, 0.0, 4.0 / 3.0 * t54 * t137);
        let t141 = t4 * t140;
        let t142 = t141 * t59;
        let t146 = t62 * t68 * t43;
        let t149 = tau1 * tau1;
        let t150 = 1.0 / t149;
        let t151 = t26 * t150;
        let t152 = t57 * t151;
        let t153 = 1.0 / t61;
        let t155 = t153 * sigma2 * t43;
        let t159 = piecewise3(t48, 0.0, -27.0 / 80.0 * t142 * t70 - t120 - 9.0 / 16.0 * t60 * t146 + 7.0 / 640.0 * t152 * t155);
        let tvrho1 = t47 + t73 + t5 * (t135 + t159);
        vrho[ip * 2 + 1] += tvrho1;
        let t162 = t25 * t26;
        let t164 = t97 * t31 * t43;
        let t167 = piecewise3(t2, 0.0, -7.0 / 640.0 * t162 * t164);
        let tvsigma0 = t5 * t167;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t168 = t57 * t26;
        let t170 = t150 * t62 * t43;
        let t173 = piecewise3(t48, 0.0, -7.0 / 640.0 * t168 * t170);
        let tvsigma2 = t5 * t173;
        vsigma[ip * 3 + 2] += tvsigma2;
        let tvlapl0 = 0.0;
        vlapl[ip * 2] += tvlapl0;
        let tvlapl1 = 0.0;
        vlapl[ip * 2 + 1] += tvlapl1;
        let t177 = 1.0 / t96 / tau0;
        let t178 = t26 * t177;
        let t179 = t25 * t178;
        let t181 = t31 * sigma0 * t43;
        let t185 = piecewise3(t2, 0.0, 27.0 / 80.0 * t99 * t44 + 7.0 / 640.0 * t179 * t181);
        let tvtau0 = t5 * t185;
        vtau[ip * 2] += tvtau0;
        let t189 = 1.0 / t149 / tau1;
        let t190 = t26 * t189;
        let t191 = t57 * t190;
        let t193 = t62 * sigma2 * t43;
        let t197 = piecewise3(t48, 0.0, 27.0 / 80.0 * t152 * t70 + 7.0 / 640.0 * t191 * t193);
        let tvtau1 = t5 * t197;
        vtau[ip * 2 + 1] += tvtau1;
    }
}
