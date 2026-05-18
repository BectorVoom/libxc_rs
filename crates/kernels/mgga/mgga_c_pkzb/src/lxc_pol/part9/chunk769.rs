//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 769/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk769<F: Float>(t135: F, t144: F, t2718: F, t5028: F, t5040: F, t5066: F, t5069: F, t5073: F, t5196: F, t5217: F, t5324: F, t5326: F, t5329: F, t5333: F, t5338: F, t5340: F, t5344: F, t5466: F, t560: F, t568: F, t639: F) -> F {
    let t5470 = t135 * t144 * t5466 * t639 + F::new(3.0) * t135 * t5217 * t560 + F::new(18.0) * t2718 * t5196 * t568 + t5028 + t5040 + t5066 - t5069 - t5073 - t5324 + t5326 - t5329 + t5333 - t5338 - t5340 - t5344;
    t5470
}
