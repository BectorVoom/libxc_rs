//! MGGA_X_MSB vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_msb.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_msb_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    param_kappa: f64,
    param_b: f64,
    param_c: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = M_CBRTPI;
        let t7 = t4 / t5;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t16 = pow_1_3(t12);
        let t18 = piecewise3(t12 <= zeta_threshold, t14 * zeta_threshold, t16 * t12);
        let t19 = pow_1_3(rho[ip]);
        let t20 = t18 * t19;
        let t21 = M_CBRT6;
        let t22 = M_PI * M_PI;
        let t23 = pow_1_3(t22);
        let t24 = t23 * t23;
        let t25 = 1.0 / t24;
        let t26 = t21 * t25;
        let t27 = M_CBRT2;
        let t28 = t27 * t27;
        let t29 = sigma[ip] * t28;
        let t30 = rho[ip] * rho[ip];
        let t31 = t19 * t19;
        let t33 = 1.0 / t31 / t30;
        let t34 = t29 * t33;
        let t36 = 5.0 / 972.0 * t26 * t34;
        let t37 = param_kappa + t36;
        let t41 = param_kappa * (1.0 - param_kappa / t37);
        let t42 = tau[ip] * t28;
        let t44 = 1.0 / t31 / rho[ip];
        let t45 = t42 * t44;
        let t47 = t45 - t34 / 8.0;
        let t48 = t47 * t47;
        let t49 = t21 * t21;
        let t52 = t45 + 3.0 / 10.0 * t49 * t24;
        let t53 = t52 * t52;
        let t54 = 1.0 / t53;
        let t57 = -4.0 * t48 * t54 + 1.0;
        let t58 = t57 * t57;
        let t59 = t58 * t57;
        let t60 = t48 * t47;
        let t61 = t53 * t52;
        let t62 = 1.0 / t61;
        let t65 = t48 * t48;
        let t67 = param_b * t65 * t48;
        let t68 = t53 * t53;
        let t70 = 1.0 / t68 / t53;
        let t73 = 8.0 * t60 * t62 + 64.0 * t67 * t70 + 1.0;
        let t74 = 1.0 / t73;
        let t75 = t59 * t74;
        let t76 = param_kappa + t36 + param_c;
        let t81 = param_kappa * (1.0 - param_kappa / t76) - t41;
        let t83 = t75 * t81 + t41 + 1.0;
        let t87 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t83);
        let tzk0 = 2.0 * t87;
        zk[ip] += tzk0;
        let t89 = t18 / t31;
        let t93 = param_kappa * param_kappa;
        let t94 = t37 * t37;
        let t97 = t93 / t94 * t21;
        let t98 = t25 * sigma[ip];
        let t99 = t30 * rho[ip];
        let t101 = 1.0 / t31 / t99;
        let t102 = t28 * t101;
        let t103 = t98 * t102;
        let t104 = t97 * t103;
        let t106 = t58 * t74;
        let t107 = t47 * t54;
        let t108 = t42 * t33;
        let t112 = -5.0 / 3.0 * t108 + t29 * t101 / 3.0;
        let t115 = t48 * t62;
        let t118 = -8.0 * t107 * t112 - 40.0 / 3.0 * t115 * t108;
        let t119 = t81 * t118;
        let t122 = t73 * t73;
        let t123 = 1.0 / t122;
        let t124 = t59 * t123;
        let t127 = 1.0 / t68;
        let t128 = t60 * t127;
        let t132 = param_b * t65 * t47;
        let t133 = t70 * t112;
        let t137 = 1.0 / t68 / t61;
        let t138 = t67 * t137;
        let t141 = 40.0 * t128 * t108 + 640.0 * t138 * t108 + 24.0 * t115 * t112 + 384.0 * t132 * t133;
        let t142 = t81 * t141;
        let t144 = t76 * t76;
        let t147 = t93 / t144 * t21;
        let t150 = -10.0 / 729.0 * t147 * t103 + 10.0 / 729.0 * t104;
        let t152 = -10.0 / 729.0 * t104 + 3.0 * t106 * t119 - t124 * t142 + t75 * t150;
        let t157 = piecewise3(t3, 0.0, -t7 * t89 * t83 / 8.0 - 3.0 / 8.0 * t7 * t20 * t152);
        let tvrho0 = 2.0 * rho[ip] * t157 + 2.0 * t87;
        vrho[ip] += tvrho0;
        let t160 = t25 * t28;
        let t161 = t160 * t33;
        let t162 = t97 * t161;
        let t164 = t106 * t81;
        let t165 = t28 * t33;
        let t166 = t107 * t165;
        let t169 = t115 * t165;
        let t171 = t70 * t28;
        let t173 = t132 * t171 * t33;
        let t175 = -3.0 * t169 - 48.0 * t173;
        let t176 = t81 * t175;
        let t180 = 5.0 / 972.0 * t147 * t161 - 5.0 / 972.0 * t162;
        let t182 = 5.0 / 972.0 * t162 + 3.0 * t164 * t166 - t124 * t176 + t75 * t180;
        let t186 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t182);
        let tvsigma0 = 2.0 * rho[ip] * t186;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t188 = t28 * t44;
        let t190 = t115 * t188;
        let t192 = -8.0 * t107 * t188 + 8.0 * t190;
        let t193 = t81 * t192;
        let t202 = t137 * t28;
        let t206 = 384.0 * t132 * t171 * t44 - 384.0 * t67 * t202 * t44 - 24.0 * t128 * t188 + 24.0 * t190;
        let t207 = t81 * t206;
        let t209 = 3.0 * t106 * t193 - t124 * t207;
        let t213 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t209);
        let tvtau0 = 2.0 * rho[ip] * t213;
        vtau[ip] += tvtau0;
    }
}
