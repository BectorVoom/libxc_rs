//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1364/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1364(t1058: f64, t3068: f64, t3087: f64, t363: f64, t11065: f64, t42387: f64, t10408: f64, t1041: f64, t10485: f64, t10877: f64, t14172: f64, t14228: f64, t2250: f64, t2770: f64, t3070: f64, t3071: f64, t3073: f64, t3134: f64, t39097: f64, t42468: f64, t43317: f64, t43322: f64, t43325: f64, t43332: f64, t43336: f64, t43341: f64, t43343: f64, t43350: f64, t43352: f64, t43354: f64, t4582: f64, t884: f64, t973: f64, t974: f64) -> f64 {
    let t43358 = t1058 * t363 * t3087 * t3068;
    let t43361 = t11065 * t42387;
    let t43366 = t973 * t974 * t43317 * t39097 / 6.0_f64 + t43322 * t10485 / 128.0_f64 + 2.0_f64 / 81.0_f64 * t43325 + 5.0_f64 / 1152.0_f64 * t3070 * t10408 * t2770 * t2250 * t14228 + t43332 / 54.0_f64 + t43336 / 1728.0_f64 - 5.0_f64 / 10368.0_f64 * t43341 + t43343 * t3134 / 256.0_f64 - 5.0_f64 / 384.0_f64 * t1041 * t4582 * t14172 * t42468 + t43350 / 384.0_f64 - t43352 / 2304.0_f64 - 19.0_f64 / 1296.0_f64 * t43354 + 19.0_f64 / 216.0_f64 * t43358 * t3073 - t43361 * t3071 * t10877 * t884 / 192.0_f64;
    t43366
}
