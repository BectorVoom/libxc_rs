//! GGA_XC_TH2 fxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_xc_th2.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_xc_th2_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
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
        let t21 = pow_1_3(rho[ip]);
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
        let tvrho0 = 0.367700125 * t6 * t7 - 1.0256225 * t14 * t15 + 0.8511733333333333 * t20 * t21 - 1.2059175 * t25 * t26 + 0.304675 * t93 - 0.0037776458333333334 * t101 * t41 + 0.006123875 * t104 * t41 + 0.01226175 * t107 * t41 - 0.0089614625 * t111 * t41 + 0.0050895875 * t115 * t56 - 0.0022356770833333334 * t119 * t56 + 6.4013e-05 * t122 * sigma[ip] * t55 + 0.02586475 * t93 * t75 + 0.01551885 * t32 * t134 - 0.06602988333333333 * t137 * t75 - 0.0360163 * t79 * t134 + 0.0446562 * rho[ip] * t75 + 0.0223281 * t70 * t134;
        vrho[ip] += tvrho0;
        let t147 = 1.0 / t36 * t40;
        let t162 = t72 * t55;
        let t163 = t162 - t72;
        let tvsigma0 = -0.022665875 * t35 * t147 + 0.018371625 * t44 * t147 + 0.018392625 * t47 * t147 - 0.0089614625 * t50 * t147 - 0.0050895875 * t54 * t55 + 0.0026828125 * t63 * t55 - 9.60195e-05 * t66 * t55 + 0.01551885 * t32 * t163 - 0.0360163 * t79 * t163 + 0.0223281 * t70 * t163;
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
        let t233 = 0.0517295 * t93 * t134 + 0.01551885 * t32 * t213 - 0.13205976666666666 * t137 * t134 - 0.0360163 * t79 * t213 + 0.030641677083333332 * t6 * t100 - 0.17093708333333332 * t14 * t62 + 0.28372444444444445 * t20 * t66 - 0.60295875 * t25 * t110 + 0.20311666666666667 * t195 + 0.0893124 * rho[ip] * t134 + 0.0223281 * t70 * t213;
        let tv2rho20 = 0.003462842013888889 * t172 * t41 - 0.0051032291666666665 * t175 * t41 - 0.0081745 * t178 * t41 + 0.00448073125 * t182 * t41 - 0.010179175 * t186 * t56 + 0.004098741319444444 * t191 * t56 + 0.017243166666666667 * t195 * t75 - 0.05502490277777778 * t199 * t75 + 0.044549511666666666 * t74 - 0.0446562 * t73 + t233;
        v2rho2[ip] += tv2rho20;
        let t250 = t130 * t55;
        let t252 = -8.0 / 3.0 * t250 + 8.0 / 3.0 * t130;
        let tv2rhosigma0 = -0.0018888229166666667 * t101 * t147 + 0.0030619375 * t104 * t147 + 0.006130875 * t107 * t147 - 0.00448073125 * t111 * t147 + 0.0050895875 * t115 * t55 - 0.0022356770833333334 * t119 * t55 + 6.4013e-05 * t122 * t55 + 0.02586475 * t93 * t163 + 0.01551885 * t32 * t252 - 0.06602988333333333 * t137 * t163 - 0.0360163 * t79 * t252 + 0.0446562 * rho[ip] * t163 + 0.0223281 * t70 * t252;
        v2rhosigma[ip] += tv2rhosigma0;
        let t265 = 1.0 / t36 / sigma[ip] * t40;
        let tv2sigma20 = 0.0113329375 * t35 * t265 - 0.0091858125 * t44 * t265 - 0.0091963125 * t47 * t265 + 0.00448073125 * t50 * t265;
        v2sigma2[ip] += tv2sigma20;
    }
}
