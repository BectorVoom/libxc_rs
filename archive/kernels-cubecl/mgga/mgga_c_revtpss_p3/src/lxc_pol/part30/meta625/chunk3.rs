//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2165/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2165<F: Float>(t2453: F, t27212: F, t25301: F, t25410: F, t7774: F, t93240: F, t14662: F, t1949: F, t231: F, t27350: F, t27354: F, t4423: F, t7048: F, t7070: F, t7076: F, t92917: F, t93180: F, t93184: F, t93192: F, t93195: F, t99228: F, t99231: F, t99234: F, t99237: F, t99243: F, t99245: F) -> F {
    let t99257 = t2453 * t27212;
    let t99258 = t99257 * t25301;
    let t99261 = t93240 * t25410 * t7774;
    let t99264 = F::cast_from(0.39029762157531132075e-1_f64) * t99228 + t99231 + F::cast_from(0.72280234901709995518e-2_f64) * t93180 + t99234 - F::cast_from(0.17347256376410398924e1_f64) * t92917 * t27350 + F::cast_from(0.8673628188205199462e0_f64) * t99237 * t27354 + F::cast_from(0.19274729307122665471e-1_f64) * t93184 - t99243 + t99245 + F::cast_from(0.91399340044406952588e-2_f64) * t93192 + F::cast_from(0.8673628188205199462e0_f64) * t7070 * t7076 * t7048 * t4423 * t231 + F::cast_from(0.4336814094102599731e0_f64) * t7070 * t7076 * t1949 * t14662 * t231 + F::cast_from(0.17135234354032049604e-2_f64) * t99258 + F::cast_from(0.24093411633903331839e-3_f64) * t99261 - F::cast_from(0.23131639038696784278e-2_f64) * t93195;
    t99264
}
