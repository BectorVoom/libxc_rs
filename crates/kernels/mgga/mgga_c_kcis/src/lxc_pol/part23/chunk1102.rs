//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1102/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1102<F: Float>(t1593: F, t28352: F, t498: F, t27369: F, t27370: F, t4007: F, t5885: F, t12234: F, t1938: F, t3715: F, t28419: F, t52649: F, t7908: F, t1928: F, t3961: F, t990: F) -> (F, F, F, F, F, F) {
    let t98137 = t1593 * t498 * t28352;
    let t98138 = t27369 * t98137;
    let t98141 = t27370 * t5885 * t4007;
    let t98144 = t12234 * t1938;
    let t98146 = t27370 * t98144 * t3715;
    let t98150 = t7908 * t52649 * t28419;
    let t98155 = t3961 * t1928 * t990;
    (t98137, t98138, t98141, t98146, t98150, t98155)
}
