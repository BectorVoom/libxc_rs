//! MGGA_X_BR89_EXPLICIT exc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_br89_explicit.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT4, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_2};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_x_br89_explicit_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    param_gamma: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
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
        let t3 = rho0 + rho1;
        let t4 = 1.0 / t3;
        let t7 = 2.0 * rho0 * t4 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t11 = 2.0 * rho1 * t4 <= zeta_threshold;
        let t12 = -t8;
        let t13 = rho0 - rho1;
        let t15 = piecewise5(t7, t8, t11, t12, t13 * t4);
        let t16 = 1.0 + t15;
        let t17 = t16 <= zeta_threshold;
        let t18 = pow_1_3(zeta_threshold);
        let t19 = t18 * zeta_threshold;
        let t20 = pow_1_3(t16);
        let t22 = piecewise3(t17, t19, t20 * t16);
        let t23 = pow_1_3(t3);
        let t24 = t22 * t23;
        let t26 = pow_1_3(1.0 / M_PI);
        let t27 = 1.0 / t26;
        let t28 = t24 * t27;
        let t29 = M_CBRT4;
        let t30 = M_CBRTPI;
        let t31 = t30 * t30;
        let t32 = pow_1_3(rho0);
        let t33 = t32 * t32;
        let t35 = 1.0 / t33 / rho0;
        let t36 = lapl0 * t35;
        let t38 = tau0 * param_gamma;
        let t39 = t38 * t35;
        let t41 = param_gamma * sigma0;
        let t42 = rho0 * rho0;
        let t44 = 1.0 / t33 / t42;
        let t45 = t41 * t44;
        let t48 = f64::abs(t36 / 2.0 - 2.0 * t39 + t45 / 4.0);
        let t50 = t48 / 3.0 < 0.5e-12;
        let t54 = t36 / 6.0 - 2.0 / 3.0 * t39 + t45 / 12.0;
        let t55 = 0.0 < t54;
        let t56 = piecewise3(t55, 0.5e-12, -0.5e-12);
        let t57 = piecewise3(t50, t56, t54);
        let t60 = 2.0 / 3.0 * t31 / t57;
        let t61 = t60 <= 0.0;
        let t62 = -0.5e-12 < t60;
        let t63 = piecewise3(t62, -0.5e-12, t60);
        let t65 = 0.1525525181200953e1 * t63 + 0.4576575543602858e0;
        let t66 = f64::atan(t65);
        let t67 = -t66 + 0.4292036732051034e0;
        let t69 = t63 * t63;
        let t71 = t69 * t63;
        let t73 = t69 * t69;
        let t75 = t73 * t63;
        let t77 = 0.7566445420735584e0 - 0.2636397787137096e1 * t63 + 0.5474515996423288e1 * t69 - 0.1265730812710829e2 * t71 + 0.4125058472512136e1 * t73 - 0.3042513395716384e2 * t75;
        let t78 = t67 * t77;
        let t84 = 0.4771976183772063e0 - 0.1779981349455627e1 * t63 + 0.3843384186230215e1 * t69 - 0.9591205088051849e1 * t71 + 0.2173018028591672e1 * t73 - 0.3042513385160366e2 * t75;
        let t85 = 1.0 / t84;
        let t87 = 0.5e-12 < t60;
        let t88 = piecewise3(t87, t60, 0.5e-12);
        let t90 = f64::ln(1.0 / (0.2085749716493756e1 * t88) + f64::sqrt(pow_2(1.0 / (0.2085749716493756e1 * t88)) + 1.0));
        let t91 = t90 + 2.0;
        let t93 = t88 * t88;
        let t95 = t93 * t88;
        let t97 = t93 * t93;
        let t99 = t97 * t88;
        let t101 = 0.4435009886795587e-4 + 0.5812865360445791e0 * t88 + 0.6674276451594061e2 * t93 + 0.4342678089722977e3 * t95 + 0.8247765766052239e3 * t97 + 0.1657965273158212e4 * t99;
        let t102 = t91 * t101;
        let t108 = 0.3347285060926091e-4 + 0.4791793102397135e0 * t88 + 0.6239226833857424e2 * t93 + 0.4631481642793812e3 * t95 + 0.7852360350104029e3 * t97 + 0.1657962968223273e4 * t99;
        let t109 = 1.0 / t108;
        let t111 = piecewise3(t61, t78 * t85, t102 * t109);
        let t113 = f64::exp(t111 / 3.0);
        let t114 = t29 * t113;
        let t115 = f64::exp(-t111);
        let t117 = 1.0 + t111 / 2.0;
        let t118 = t115 * t117;
        let t119 = 1.0 - t118;
        let t120 = 1.0 / t111;
        let t121 = t119 * t120;
        let t122 = t114 * t121;
        let t125 = piecewise3(t2, 0.0, -t28 * t122 / 4.0);
        let t126 = rho1 <= dens_threshold;
        let t127 = -t13;
        let t129 = piecewise5(t11, t8, t7, t12, t127 * t4);
        let t130 = 1.0 + t129;
        let t131 = t130 <= zeta_threshold;
        let t132 = pow_1_3(t130);
        let t134 = piecewise3(t131, t19, t132 * t130);
        let t135 = t134 * t23;
        let t136 = t135 * t27;
        let t137 = pow_1_3(rho1);
        let t138 = t137 * t137;
        let t140 = 1.0 / t138 / rho1;
        let t141 = lapl1 * t140;
        let t143 = tau1 * param_gamma;
        let t144 = t143 * t140;
        let t146 = param_gamma * sigma2;
        let t147 = rho1 * rho1;
        let t149 = 1.0 / t138 / t147;
        let t150 = t146 * t149;
        let t153 = f64::abs(t141 / 2.0 - 2.0 * t144 + t150 / 4.0);
        let t155 = t153 / 3.0 < 0.5e-12;
        let t159 = t141 / 6.0 - 2.0 / 3.0 * t144 + t150 / 12.0;
        let t160 = 0.0 < t159;
        let t161 = piecewise3(t160, 0.5e-12, -0.5e-12);
        let t162 = piecewise3(t155, t161, t159);
        let t165 = 2.0 / 3.0 * t31 / t162;
        let t166 = t165 <= 0.0;
        let t167 = -0.5e-12 < t165;
        let t168 = piecewise3(t167, -0.5e-12, t165);
        let t170 = 0.1525525181200953e1 * t168 + 0.4576575543602858e0;
        let t171 = f64::atan(t170);
        let t172 = -t171 + 0.4292036732051034e0;
        let t174 = t168 * t168;
        let t176 = t174 * t168;
        let t178 = t174 * t174;
        let t180 = t178 * t168;
        let t182 = 0.7566445420735584e0 - 0.2636397787137096e1 * t168 + 0.5474515996423288e1 * t174 - 0.1265730812710829e2 * t176 + 0.4125058472512136e1 * t178 - 0.3042513395716384e2 * t180;
        let t183 = t172 * t182;
        let t189 = 0.4771976183772063e0 - 0.1779981349455627e1 * t168 + 0.3843384186230215e1 * t174 - 0.9591205088051849e1 * t176 + 0.2173018028591672e1 * t178 - 0.3042513385160366e2 * t180;
        let t190 = 1.0 / t189;
        let t192 = 0.5e-12 < t165;
        let t193 = piecewise3(t192, t165, 0.5e-12);
        let t195 = f64::ln(1.0 / (0.2085749716493756e1 * t193) + f64::sqrt(pow_2(1.0 / (0.2085749716493756e1 * t193)) + 1.0));
        let t196 = t195 + 2.0;
        let t198 = t193 * t193;
        let t200 = t198 * t193;
        let t202 = t198 * t198;
        let t204 = t202 * t193;
        let t206 = 0.4435009886795587e-4 + 0.5812865360445791e0 * t193 + 0.6674276451594061e2 * t198 + 0.4342678089722977e3 * t200 + 0.8247765766052239e3 * t202 + 0.1657965273158212e4 * t204;
        let t207 = t196 * t206;
        let t213 = 0.3347285060926091e-4 + 0.4791793102397135e0 * t193 + 0.6239226833857424e2 * t198 + 0.4631481642793812e3 * t200 + 0.7852360350104029e3 * t202 + 0.1657962968223273e4 * t204;
        let t214 = 1.0 / t213;
        let t216 = piecewise3(t166, t183 * t190, t207 * t214);
        let t218 = f64::exp(t216 / 3.0);
        let t219 = t29 * t218;
        let t220 = f64::exp(-t216);
        let t222 = 1.0 + t216 / 2.0;
        let t223 = t220 * t222;
        let t224 = 1.0 - t223;
        let t225 = 1.0 / t216;
        let t226 = t224 * t225;
        let t227 = t219 * t226;
        let t230 = piecewise3(t126, 0.0, -t136 * t227 / 4.0);
        let tzk0 = t125 + t230;
        zk[ip] += tzk0;
    }
}
