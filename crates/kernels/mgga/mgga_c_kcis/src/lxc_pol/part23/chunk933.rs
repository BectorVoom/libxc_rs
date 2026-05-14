//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 933/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk933<F: Float>(t27332: F, t303: F, t1014: F, t7932: F, t7935: F, t12231: F, t1598: F, t12234: F, t498: F, t3715: F, t6176: F) -> (F, F, F, F, F, F) {
    let t27333 = t303 * t27332;
    let t27335 = t1014 * t7932;
    let t27337 = t1014 * t7935;
    let t27339 = t12231 * t1598;
    let t27340 = t498 * t12234;
    let t27341 = t27340 * t3715;
    let t27342 = t6176 * t27341;
    (t27333, t27335, t27337, t27339, t27341, t27342)
}
