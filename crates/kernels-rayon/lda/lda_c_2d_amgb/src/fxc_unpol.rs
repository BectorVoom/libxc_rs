//! LDA_C_2D_AMGB fxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_2d_amgb.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_3_2};

#[allow(unused_variables, non_snake_case)]
pub fn lda_c_2d_amgb_fxc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t1 = f64::sqrt(rho[ip]);
        let t2 = 1.0 / t1;
        let t4 = 1.0 / rho[ip];
        let t7 = 1.0 / t1 / rho[ip];
        let t9 = 0.04869723403850762 * t2 + 0.018219548589342285 * t4 + 0.000603947002028882 * t7;
        let t11 = f64::sqrt(M_PI);
        let t12 = 1.0 / t11;
        let t13 = t12 * t2;
        let t14 = pow_3_2(t13);
        let t18 = 0.5654308006315614 * t2 - 0.02069 * t14 + 0.10821581200590331 * t4 + 0.00313738702352666 * t7;
        let t20 = 1.0 + 1.0 / t18;
        let t21 = f64::ln(t20);
        let t22 = t9 * t21;
        let t24 = f64::exp(-0.7552241765370266 * t2);
        let t26 = M_SQRT2;
        let t27 = (t24 - 1.0) * t26;
        let t30 = f64::sqrt(zeta_threshold);
        let t32 = piecewise3(1.0 <= zeta_threshold, t30 * zeta_threshold, 1.0);
        let t33 = t32 - 1.0;
        let t36 = 4.0 / 3.0 * t27 * t12 * t1 * t33;
        let tzk0 = -0.1925 + t22 - t36;
        zk[ip] += tzk0;
        let t38 = rho[ip] * rho[ip];
        let t39 = 1.0 / t38;
        let t42 = 1.0 / t1 / t38;
        let t44 = -0.02434861701925381 * t7 - 0.018219548589342285 * t39 - 0.000905920503043323 * t42;
        let t45 = t44 * t21;
        let t46 = t18 * t18;
        let t47 = 1.0 / t46;
        let t48 = t9 * t47;
        let t50 = f64::sqrt(t13);
        let t51 = t50 * t12;
        let t56 = -0.2827154003157807 * t7 + 0.0155175 * t51 * t7 - 0.10821581200590331 * t39 - 0.00470608053528999 * t42;
        let t57 = 1.0 / t20;
        let t58 = t56 * t57;
        let t59 = t48 * t58;
        let t61 = t26 * t33;
        let t62 = t4 * t24 * t61;
        let t65 = t27 * t13 * t33;
        let tvrho0 = -0.1925 + t22 - t36 + rho[ip] * (t45 - t59 - 0.2840597424304148 * t62 - 2.0 / 3.0 * t65);
        vrho[ip] += tvrho0;
        let t74 = t38 * rho[ip];
        let t75 = 1.0 / t74;
        let t78 = 1.0 / t1 / t74;
        let t80 = 0.036522925528880715 * t42 + 0.03643909717868457 * t75 + 0.0022648012576083074 * t78;
        let t81 = t80 * t21;
        let t82 = t44 * t47;
        let t83 = t82 * t58;
        let t85 = t46 * t18;
        let t86 = 1.0 / t85;
        let t87 = t9 * t86;
        let t88 = t56 * t56;
        let t89 = t88 * t57;
        let t90 = t87 * t89;
        let t93 = 1.0/f64::sqrt(t13);
        let t95 = t93 / M_PI;
        let t102 = 0.424073100473671 * t42 - 0.003879375 * t95 * t75 - 0.02327625 * t51 * t42 + 0.21643162401180663 * t75 + 0.011765201338224974 * t78;
        let t103 = t102 * t57;
        let t104 = t48 * t103;
        let t105 = t46 * t46;
        let t106 = 1.0 / t105;
        let t107 = t9 * t106;
        let t108 = t20 * t20;
        let t109 = 1.0 / t108;
        let t110 = t88 * t109;
        let t111 = t107 * t110;
        let t113 = t39 * t24 * t61;
        let t116 = t42 * t24 * t61;
        let t120 = t27 * t12 * t7 * t33;
        let tv2rho20 = 2.0 * t45 - 2.0 * t59 - 0.5681194848608296 * t62 - 4.0 / 3.0 * t65 + rho[ip] * (t81 - 2.0 * t83 + 2.0 * t90 - t104 - t111 + 0.1420298712152074 * t113 - 0.10726439253216494 * t116 + t120 / 3.0);
        v2rho2[ip] += tv2rho20;
    }
}
