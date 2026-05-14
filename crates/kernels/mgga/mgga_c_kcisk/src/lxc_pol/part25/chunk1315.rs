//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1315/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1315<F: Float>(t5014: F, t9670: F, t112192: F, t116923: F, t17300: F, t7233: F, t112406: F, t112560: F, t116201: F, t116912: F, t116916: F, t116921: F, t116925: F, t116929: F, t17305: F, t17309: F, t32952: F, t32959: F, t33031: F, t33056: F, t34122: F, t9667: F) -> (F, F) {
    let t116932 = t5014 * t9670;
    let t116939 = t112192 * t116923 * t17300;
    let t116942 = t7233 * t9670;
    let t116952 = 0.24872916666666666666e-2 * t116912 + 0.35740740740740740742e-2 * t33056 * t116916 - t112560 - t116921 - 0.15520416666666666667e-2 * t112406 * t116925 - 0.69444444444444444446e-2 * t33031 * t116929 - 0.13888888888888888889e-1 * t33031 * t116932 * t17305 - 0.80416666666666666667e-2 * t33056 * t116925 - 0.26805555555555555556e-2 * t33056 * t116939 + 0.92592592592592592594e-2 * t33031 * t116942 * t17309 - 0.69444444444444444446e-2 * t116201 * t9667 - 0.34722222222222222223e-2 * t34122 * t32959 - 0.46296296296296296297e-2 * t34122 * t32952;
    (t116939, t116952)
}
