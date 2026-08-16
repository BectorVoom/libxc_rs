//! GGA_XC_TH2 fxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_xc_th2.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_2_3, pow_4_3, pow_5_3, pow_7_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_xc_th2_fxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v2rhosigma: &mut Array<f64>,
    v2sigma2: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = f64::powf(2.0, 1.0 / 12.0);
        let t2 = t1 * t1;
        let t3 = t2 * t1;
        let t4 = t2 * t2;
        let t5 = t4 * t4;
        let t6 = t5 * t3;
        let t7 = f64::powf(rho[ip], 1.0 / 12.0);
        let t11 = f64::powf(2.0, 1.0 / 6.0);
        let t12 = t11 * t11;
        let t13 = t12 * t12;
        let t14 = t13 * t11;
        let t15 = f64::powf(rho[ip], 1.0 / 6.0);
        let t16 = t15 * rho[ip];
        let t19 = M_CBRT2;
        let t20 = t19 * t19;
        let t21 = pow_1_3::<f64>(rho[ip]);
        let t22 = t21 * rho[ip];
        let t25 = M_SQRT2;
        let t26 = f64::sqrt(rho[ip]);
        let t27 = t26 * rho[ip];
        let t30 = t21 * t21;
        let t31 = t30 * rho[ip];
        let t32 = t19 * t31;
        let t34 = t4 * t3;
        let t35 = t34 * t7;
        let t36 = f64::sqrt(sigma[ip]);
        let t38 = pow_1_3::<f64>(zeta_threshold);
        let t40 = piecewise3::<f64>(1.0 <= zeta_threshold, t38 * zeta_threshold, 1.0);
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
        let t84 = 0.3394155e0 * t6 * t7 * rho[ip] - 0.879105e0 * t14 * t16 + 0.63838e0 * t20 * t22 - 0.803945e0 * t25 * t27 + 0.182805e0 * t32 - 0.4533175e-1 * t35 * t41 + 0.3674325e-1 * t44 * t41 + 0.3678525e-1 * t47 * t41 - 0.17922925e-1 * t50 * t41 - 0.50895875e-2 * t54 * t56 + 0.26828125e-2 * t63 * t56 - 0.960195e-4 * t66 * sigma[ip] * t55 + 0.1551885e-1 * t32 * t75 - 0.360163e-1 * t79 * t75 + 0.223281e-1 * t70 * t75;
        let tzk0 = t84 * t53;
        zk[ip] += tzk0;
        let t93 = t19 * t30;
        let t95 = t7 * t7;
        let t97 = t95 * t95;
        let t98 = t97 * t97;
        let t99 = t98 * t95 * t7;
        let t100 = 1.0 / t99;
        let t101 = t34 * t100;
        let t104 = t25 * t62;
        let t107 = t19 * t66;
        let t110 = 1.0 / t26;
        let t111 = t11 * t110;
        let t115 = t19 / t70;
        let t118 = 1.0 / t78;
        let t119 = t11 * t118;
        let t122 = 1.0 / t31;
        let t128 = t70 * rho[ip];
        let t130 = 1.0 / t30 / t128;
        let t131 = sigma[ip] * t130;
        let t132 = t131 * t55;
        let t134 = -8.0 / 3.0 * t132 + 8.0 / 3.0 * t131;
        let t137 = t11 * t61;
        let tvrho0 = 0.367700125e0 * t6 * t7 - 0.10256225e1 * t14 * t15 + 0.85117333333333333333e0 * t20 * t21 - 0.12059175e1 * t25 * t26 + 0.304675e0 * t93 - 0.37776458333333333333e-2 * t101 * t41 + 0.6123875e-2 * t104 * t41 + 0.1226175e-1 * t107 * t41 - 0.89614625e-2 * t111 * t41 + 0.50895875e-2 * t115 * t56 - 0.22356770833333333333e-2 * t119 * t56 + 0.64013e-4 * t122 * sigma[ip] * t55 + 0.2586475e-1 * t93 * t75 + 0.1551885e-1 * t32 * t134 - 0.66029883333333333333e-1 * t137 * t75 - 0.360163e-1 * t79 * t134 + 0.446562e-1 * rho[ip] * t75 + 0.223281e-1 * t70 * t134;
        vrho[ip] += tvrho0;
        let t147 = 1.0 / t36 * t40;
        let t162 = t72 * t55;
        let t163 = t162 - t72;
        let tvsigma0 = -0.22665875e-1 * t35 * t147 + 0.18371625e-1 * t44 * t147 + 0.18392625e-1 * t47 * t147 - 0.89614625e-2 * t50 * t147 - 0.50895875e-2 * t54 * t55 + 0.26828125e-2 * t63 * t55 - 0.960195e-4 * t66 * t55 + 0.1551885e-1 * t32 * t163 - 0.360163e-1 * t79 * t163 + 0.223281e-1 * t70 * t163;
        vsigma[ip] += tvsigma0;
        let t171 = 1.0 / t99 / rho[ip];
        let t172 = t34 * t171;
        let t175 = t25 * t118;
        let t178 = t19 * t122;
        let t181 = 1.0 / t27;
        let t182 = t11 * t181;
        let t186 = t19 / t128;
        let t190 = 1.0 / t61 / t70;
        let t191 = t11 * t190;
        let t195 = t19 / t21;
        let t199 = t11 / t15;
        let t207 = t70 * t70;
        let t209 = 1.0 / t30 / t207;
        let t210 = sigma[ip] * t209;
        let t211 = t210 * t55;
        let t213 = 88.0 / 9.0 * t211 - 88.0 / 9.0 * t210;
        let t233 = 0.517295e-1 * t93 * t134 + 0.1551885e-1 * t32 * t213 - 0.13205976666666666667e0 * t137 * t134 - 0.360163e-1 * t79 * t213 + 0.30641677083333333333e-1 * t6 * t100 - 0.17093708333333333333e0 * t14 * t62 + 0.28372444444444444444e0 * t20 * t66 - 0.60295875e0 * t25 * t110 + 0.20311666666666666667e0 * t195 + 0.893124e-1 * rho[ip] * t134 + 0.223281e-1 * t70 * t213;
        let tv2rho20 = 0.34628420138888888889e-2 * t172 * t41 - 0.51032291666666666667e-2 * t175 * t41 - 0.81745e-2 * t178 * t41 + 0.448073125e-2 * t182 * t41 - 0.10179175e-1 * t186 * t56 + 0.40987413194444444444e-2 * t191 * t56 + 0.17243166666666666667e-1 * t195 * t75 - 0.55024902777777777777e-1 * t199 * t75 + 0.44549511666666666667e-1 * t74 - 0.446562e-1 * t73 + t233;
        v2rho2[ip] += tv2rho20;
        let t250 = t130 * t55;
        let t252 = -8.0 / 3.0 * t250 + 8.0 / 3.0 * t130;
        let tv2rhosigma0 = -0.18888229166666666667e-2 * t101 * t147 + 0.30619375e-2 * t104 * t147 + 0.6130875e-2 * t107 * t147 - 0.448073125e-2 * t111 * t147 + 0.50895875e-2 * t115 * t55 - 0.22356770833333333333e-2 * t119 * t55 + 0.64013e-4 * t122 * t55 + 0.2586475e-1 * t93 * t163 + 0.1551885e-1 * t32 * t252 - 0.66029883333333333333e-1 * t137 * t163 - 0.360163e-1 * t79 * t252 + 0.446562e-1 * rho[ip] * t163 + 0.223281e-1 * t70 * t252;
        v2rhosigma[ip] += tv2rhosigma0;
        let t265 = 1.0 / t36 / sigma[ip] * t40;
        let tv2sigma20 = 0.113329375e-1 * t35 * t265 - 0.91858125e-2 * t44 * t265 - 0.91963125e-2 * t47 * t265 + 0.448073125e-2 * t50 * t265;
        v2sigma2[ip] += tv2sigma20;
    }
}
