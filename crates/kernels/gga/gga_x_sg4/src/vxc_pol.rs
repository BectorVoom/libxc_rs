//! GGA_X_SG4 vxc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_sg4.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_x_sg4_vxc_pol(
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
        let t1 = rho0 <= dens_threshold;
        let t2 = M_CBRT3;
        let t3 = M_CBRTPI;
        let t5 = t2 / t3;
        let t6 = rho0 + rho1;
        let t7 = 1.0 / t6;
        let t10 = 2.0 * rho0 * t7 <= zeta_threshold;
        let t11 = zeta_threshold - 1.0;
        let t14 = 2.0 * rho1 * t7 <= zeta_threshold;
        let t15 = -t11;
        let t16 = rho0 - rho1;
        let t18 = piecewise5::<f64>(t10, t11, t14, t15, t16 * t7);
        let t19 = 1.0 + t18;
        let t20 = t19 <= zeta_threshold;
        let t21 = pow_1_3::<f64>(zeta_threshold);
        let t22 = t21 * zeta_threshold;
        let t23 = pow_1_3::<f64>(t19);
        let t25 = piecewise3::<f64>(t20, t22, t23 * t19);
        let t26 = pow_1_3::<f64>(t6);
        let t27 = t25 * t26;
        let t28 = M_CBRT6;
        let t29 = M_PI * M_PI;
        let t30 = pow_1_3::<f64>(t29);
        let t31 = t30 * t30;
        let t32 = 1.0 / t31;
        let t33 = t28 * t32;
        let t34 = rho0 * rho0;
        let t35 = pow_1_3::<f64>(rho0);
        let t36 = t35 * t35;
        let t38 = 1.0 / t36 / t34;
        let t40 = t33 * sigma0 * t38;
        let t42 = 1.0 - 0.3123398257303946694e-2 * t40;
        let t43 = t28 * t28;
        let t44 = t29 * t29;
        let t45 = t44 * t29;
        let t47 = 1.0 / t30 / t45;
        let t48 = t43 * t47;
        let t49 = sigma0 * sigma0;
        let t50 = t49 * t49;
        let t51 = t50 * sigma0;
        let t52 = t34 * t34;
        let t53 = t52 * rho0;
        let t54 = t52 * t52;
        let t55 = t54 * t53;
        let t57 = 1.0 / t35 / t55;
        let t61 = 1.0 - 0.17835614159590036509e-11 * t48 * t51 * t57;
        let t62 = 1.0 / t61;
        let t66 = 1.0 + 0.37270642201834862386e-1 * t40;
        let t69 = 0.1804e1 - 0.56028717948717948718e0 * t42 * t62 - 0.24371282051282051282e0 / t66;
        let t73 = piecewise3::<f64>(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t69);
        let t74 = rho1 <= dens_threshold;
        let t75 = -t16;
        let t77 = piecewise5::<f64>(t14, t11, t10, t15, t75 * t7);
        let t78 = 1.0 + t77;
        let t79 = t78 <= zeta_threshold;
        let t80 = pow_1_3::<f64>(t78);
        let t82 = piecewise3::<f64>(t79, t22, t80 * t78);
        let t83 = t82 * t26;
        let t84 = rho1 * rho1;
        let t85 = pow_1_3::<f64>(rho1);
        let t86 = t85 * t85;
        let t88 = 1.0 / t86 / t84;
        let t90 = t33 * sigma2 * t88;
        let t92 = 1.0 - 0.3123398257303946694e-2 * t90;
        let t93 = sigma2 * sigma2;
        let t94 = t93 * t93;
        let t95 = t94 * sigma2;
        let t96 = t84 * t84;
        let t97 = t96 * rho1;
        let t98 = t96 * t96;
        let t99 = t98 * t97;
        let t101 = 1.0 / t85 / t99;
        let t105 = 1.0 - 0.17835614159590036509e-11 * t48 * t95 * t101;
        let t106 = 1.0 / t105;
        let t110 = 1.0 + 0.37270642201834862386e-1 * t90;
        let t113 = 0.1804e1 - 0.56028717948717948718e0 * t92 * t106 - 0.24371282051282051282e0 / t110;
        let t117 = piecewise3::<f64>(t74, 0.0, -3.0 / 8.0 * t5 * t83 * t113);
        let tzk0 = t73 + t117;
        zk[ip] += tzk0;
        let t118 = t6 * t6;
        let t119 = 1.0 / t118;
        let t120 = t16 * t119;
        let t122 = piecewise5::<f64>(t10, 0.0, t14, 0.0, t7 - t120);
        let t125 = piecewise3::<f64>(t20, 0.0, 4.0 / 3.0 * t23 * t122);
        let t126 = t125 * t26;
        let t130 = t26 * t26;
        let t131 = 1.0 / t130;
        let t132 = t25 * t131;
        let t135 = t5 * t132 * t69 / 8.0;
        let t136 = t34 * rho0;
        let t138 = 1.0 / t36 / t136;
        let t143 = t61 * t61;
        let t144 = 1.0 / t143;
        let t146 = t42 * t144 * t43;
        let t147 = t47 * t51;
        let t148 = t52 * t34;
        let t149 = t54 * t148;
        let t151 = 1.0 / t35 / t149;
        let t155 = t66 * t66;
        let t157 = 1.0 / t155 * t28;
        let t158 = t32 * sigma0;
        let t162 = -0.46666666666666666667e-2 * t33 * sigma0 * t138 * t62 + 0.13324087935864403616e-10 * t146 * t147 * t151 - 0.24222222222222222223e-1 * t157 * t158 * t138;
        let t167 = piecewise3::<f64>(t1, 0.0, -3.0 / 8.0 * t5 * t126 * t69 - t135 - 3.0 / 8.0 * t5 * t27 * t162);
        let t168 = t75 * t119;
        let t170 = piecewise5::<f64>(t14, 0.0, t10, 0.0, -t7 - t168);
        let t173 = piecewise3::<f64>(t79, 0.0, 4.0 / 3.0 * t80 * t170);
        let t174 = t173 * t26;
        let t178 = t82 * t131;
        let t181 = t5 * t178 * t113 / 8.0;
        let t183 = piecewise3::<f64>(t74, 0.0, -3.0 / 8.0 * t5 * t174 * t113 - t181);
        let tvrho0 = t73 + t117 + t6 * (t167 + t183);
        vrho[ip * 2] += tvrho0;
        let t187 = piecewise5::<f64>(t10, 0.0, t14, 0.0, -t7 - t120);
        let t190 = piecewise3::<f64>(t20, 0.0, 4.0 / 3.0 * t23 * t187);
        let t191 = t190 * t26;
        let t196 = piecewise3::<f64>(t1, 0.0, -3.0 / 8.0 * t5 * t191 * t69 - t135);
        let t198 = piecewise5::<f64>(t14, 0.0, t10, 0.0, t7 - t168);
        let t201 = piecewise3::<f64>(t79, 0.0, 4.0 / 3.0 * t80 * t198);
        let t202 = t201 * t26;
        let t206 = t84 * rho1;
        let t208 = 1.0 / t86 / t206;
        let t213 = t105 * t105;
        let t214 = 1.0 / t213;
        let t216 = t92 * t214 * t43;
        let t217 = t47 * t95;
        let t218 = t96 * t84;
        let t219 = t98 * t218;
        let t221 = 1.0 / t85 / t219;
        let t225 = t110 * t110;
        let t227 = 1.0 / t225 * t28;
        let t228 = t32 * sigma2;
        let t232 = -0.46666666666666666667e-2 * t33 * sigma2 * t208 * t106 + 0.13324087935864403616e-10 * t216 * t217 * t221 - 0.24222222222222222223e-1 * t227 * t228 * t208;
        let t237 = piecewise3::<f64>(t74, 0.0, -3.0 / 8.0 * t5 * t202 * t113 - t181 - 3.0 / 8.0 * t5 * t83 * t232);
        let tvrho1 = t73 + t117 + t6 * (t196 + t237);
        vrho[ip * 2 + 1] += tvrho1;
        let t243 = t47 * t50;
        let t250 = 0.175e-2 * t33 * t38 * t62 - 0.4996532975949151356e-11 * t146 * t243 * t57 + 0.90833333333333333335e-2 * t157 * t32 * t38;
        let t254 = piecewise3::<f64>(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t250);
        let tvsigma0 = t6 * t254;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t258 = t47 * t94;
        let t265 = 0.175e-2 * t33 * t88 * t106 - 0.4996532975949151356e-11 * t216 * t258 * t101 + 0.90833333333333333335e-2 * t227 * t32 * t88;
        let t269 = piecewise3::<f64>(t74, 0.0, -3.0 / 8.0 * t5 * t83 * t265);
        let tvsigma2 = t6 * t269;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
