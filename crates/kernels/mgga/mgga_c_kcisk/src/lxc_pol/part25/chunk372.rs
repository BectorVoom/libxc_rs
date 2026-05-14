//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 372/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk372<F: Float>(t1674: F, t2370: F, t2384: F, t2387: F, t2396: F, t45: F) -> (F,) {
    let t2399 = -t2370 + t2384 + 0.19751789702565206229e-1 * t45 * t2387 - 0.58482233974552040708e0 * t1674 * t2396;
    (t2399,)
}
