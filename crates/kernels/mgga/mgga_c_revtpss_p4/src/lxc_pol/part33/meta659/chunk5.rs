//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2133/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2133<F: Float>(t6072: F, t689: F, t7014: F, t5978: F, t886: F, t1558: F, t231: F, t4533: F, t25391: F, t25392: F, t27199: F, t27292: F, t27313: F, t27350: F, t27353: F, t62624: F, t62637: F, t93252: F, t93272: F, t93273: F, t99191: F, t99307: F, t99313: F, t99323: F, t99342: F) -> F {
    let t106286 = t689 * t7014 * t6072;
    let t106290 = t5978 * t886;
    let t106302 = t4533 * t1558 * t231;
    let t106313 = F::cast_from(0.54878743191129263322e-2_f64) * t106286 - F::cast_from(0.17347256376410398924e1_f64) * t99191 * t27313 - F::cast_from(0.8673628188205199462e0_f64) * t25391 * t25392 * t106290 + F::cast_from(0.8673628188205199462e0_f64) * t27353 * t25392 * t62624 + F::cast_from(0.11565819519348392139e-2_f64) * t93252 - F::cast_from(0.17347256376410398924e1_f64) * t99191 * t27350 - F::cast_from(0.26019841438354088051e-1_f64) * t99307 - F::cast_from(0.17347256376410398924e1_f64) * t25391 * t25392 * t106302 + F::cast_from(0.4336814094102599731e0_f64) * t27353 * t25392 * t62637 - F::cast_from(0.23131639038696784278e-2_f64) * t99313 + t99323 + F::cast_from(0.8673628188205199462e0_f64) * t27199 * t27292 + t93272 + F::cast_from(0.13009920719177044025e-1_f64) * t93273 + t99342;
    t106313
}
