//! GGA_X_2D_B88 vxc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 60 shared lines across all orders.
//! Delta: 68 lines unique to vxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_2};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_2d_b88_vxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
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
        // --- shared preamble (60 lines) ---
        let t1 = rho0 <= dens_threshold;
        let t2 = f64::sqrt(M_PI);
        let t3 = 1.0 / t2;
        let t4 = rho0 + rho1;
        let t5 = 1.0 / t4;
        let t8 = 2.0 * rho0 * t5 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t12 = 2.0 * rho1 * t5 <= zeta_threshold;
        let t13 = -t9;
        let t14 = rho0 - rho1;
        let t16 = piecewise5(t8, t9, t12, t13, t14 * t5);
        let t17 = 1.0 + t16;
        let t18 = t17 <= zeta_threshold;
        let t19 = f64::sqrt(zeta_threshold);
        let t20 = t19 * zeta_threshold;
        let t21 = f64::sqrt(t17);
        let t22 = t21 * t17;
        let t23 = piecewise3(t18, t20, t22);
        let t24 = t3 * t23;
        let t25 = M_SQRT2;
        let t26 = f64::sqrt(t4);
        let t27 = t25 * t26;
        let t28 = rho0 * rho0;
        let t29 = t28 * rho0;
        let t30 = 1.0 / t29;
        let t31 = sigma0 * t30;
        let t32 = f64::sqrt(sigma0);
        let t33 = f64::sqrt(rho0);
        let t35 = 1.0 / t33 / rho0;
        let t36 = t32 * t35;
        let t37 = f64::ln(t36 + f64::sqrt(t36 * t36 + 1.0));
        let t40 = 1.0 + 0.56e-1 * t36 * t37;
        let t41 = 1.0 / t40;
        let t44 = 1.0 + 0.46526913586269795717e-2 * t31 * t41;
        let t45 = t27 * t44;
        let t48 = piecewise3(t1, 0.0, -2.0 / 3.0 * t24 * t45);
        let t49 = rho1 <= dens_threshold;
        let t50 = -t14;
        let t52 = piecewise5(t12, t9, t8, t13, t50 * t5);
        let t53 = 1.0 + t52;
        let t54 = t53 <= zeta_threshold;
        let t55 = f64::sqrt(t53);
        let t56 = t55 * t53;
        let t57 = piecewise3(t54, t20, t56);
        let t58 = t3 * t57;
        let t59 = rho1 * rho1;
        let t60 = t59 * rho1;
        let t61 = 1.0 / t60;
        let t62 = sigma2 * t61;
        let t63 = f64::sqrt(sigma2);
        let t64 = f64::sqrt(rho1);
        let t66 = 1.0 / t64 / rho1;
        let t67 = t63 * t66;
        let t68 = f64::ln(t67 + f64::sqrt(t67 * t67 + 1.0));
        let t71 = 1.0 + 0.56e-1 * t67 * t68;
        let t72 = 1.0 / t71;
        let t75 = 1.0 + 0.46526913586269795717e-2 * t62 * t72;
        let t76 = t27 * t75;
        let t79 = piecewise3(t49, 0.0, -2.0 / 3.0 * t58 * t76);
        let tzk0 = t48 + t79;
        zk[ip] += tzk0;
        // --- vxc delta (this level) (68 lines) ---
        let t80 = t4 * t4;
        let t81 = 1.0 / t80;
        let t82 = t14 * t81;
        let t84 = piecewise5(t8, 0.0, t12, 0.0, t5 - t82);
        let t87 = piecewise3(t18, 0.0, 3.0 / 2.0 * t21 * t84);
        let t88 = t3 * t87;
        let t92 = t25 / t26;
        let t93 = t92 * t44;
        let t95 = t24 * t93 / 3.0;
        let t96 = t28 * t28;
        let t97 = 1.0 / t96;
        let t98 = sigma0 * t97;
        let t101 = t40 * t40;
        let t102 = 1.0 / t101;
        let t104 = 1.0 / t33 / t28;
        let t108 = t31 + 1.0;
        let t109 = f64::sqrt(t108);
        let t110 = 1.0 / t109;
        let t113 = -0.84e-1 * t32 * t104 * t37 - 0.84e-1 * t98 * t110;
        let t114 = t102 * t113;
        let t117 = -0.13958074075880938715e-1 * t98 * t41 - 0.46526913586269795717e-2 * t31 * t114;
        let t118 = t27 * t117;
        let t122 = piecewise3(t1, 0.0, -2.0 / 3.0 * t88 * t45 - t95 - 2.0 / 3.0 * t24 * t118);
        let t123 = t50 * t81;
        let t125 = piecewise5(t12, 0.0, t8, 0.0, -t5 - t123);
        let t128 = piecewise3(t54, 0.0, 3.0 / 2.0 * t55 * t125);
        let t129 = t3 * t128;
        let t132 = t92 * t75;
        let t134 = t58 * t132 / 3.0;
        let t136 = piecewise3(t49, 0.0, -2.0 / 3.0 * t129 * t76 - t134);
        let tvrho0 = t48 + t79 + t4 * (t122 + t136);
        vrho[ip * 2] += tvrho0;
        let t140 = piecewise5(t8, 0.0, t12, 0.0, -t5 - t82);
        let t143 = piecewise3(t18, 0.0, 3.0 / 2.0 * t21 * t140);
        let t144 = t3 * t143;
        let t148 = piecewise3(t1, 0.0, -2.0 / 3.0 * t144 * t45 - t95);
        let t150 = piecewise5(t12, 0.0, t8, 0.0, t5 - t123);
        let t153 = piecewise3(t54, 0.0, 3.0 / 2.0 * t55 * t150);
        let t154 = t3 * t153;
        let t157 = t59 * t59;
        let t158 = 1.0 / t157;
        let t159 = sigma2 * t158;
        let t162 = t71 * t71;
        let t163 = 1.0 / t162;
        let t165 = 1.0 / t64 / t59;
        let t169 = t62 + 1.0;
        let t170 = f64::sqrt(t169);
        let t171 = 1.0 / t170;
        let t174 = -0.84e-1 * t63 * t165 * t68 - 0.84e-1 * t159 * t171;
        let t175 = t163 * t174;
        let t178 = -0.13958074075880938715e-1 * t159 * t72 - 0.46526913586269795717e-2 * t62 * t175;
        let t179 = t27 * t178;
        let t183 = piecewise3(t49, 0.0, -2.0 / 3.0 * t154 * t76 - t134 - 2.0 / 3.0 * t58 * t179);
        let tvrho1 = t48 + t79 + t4 * (t148 + t183);
        vrho[ip * 2 + 1] += tvrho1;
        let t188 = 1.0 / t32;
        let t194 = 0.28e-1 * t188 * t35 * t37 + 0.28e-1 * t30 * t110;
        let t195 = t102 * t194;
        let t198 = 0.46526913586269795717e-2 * t30 * t41 - 0.46526913586269795717e-2 * t31 * t195;
        let t199 = t27 * t198;
        let t202 = piecewise3(t1, 0.0, -2.0 / 3.0 * t24 * t199);
        let tvsigma0 = t4 * t202;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t205 = 1.0 / t63;
        let t211 = 0.28e-1 * t205 * t66 * t68 + 0.28e-1 * t61 * t171;
        let t212 = t163 * t211;
        let t215 = 0.46526913586269795717e-2 * t61 * t72 - 0.46526913586269795717e-2 * t62 * t212;
        let t216 = t27 * t215;
        let t219 = piecewise3(t49, 0.0, -2.0 / 3.0 * t58 * t216);
        let tvsigma2 = t4 * t219;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
