//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 595/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk595<F: Float>(t2572: F, t2573: F, t360: F, t2551: F, t2132: F, t571: F) -> (F, F, F, F, F) {
    let t2574 = t2572 * t2573;
    let t2575 = t360 * t2574;
    let t2578 = t2572 * t2551;
    let t2579 = t360 * t2578;
    let t2582 = t571 * t2132;
    (t2574, t2575, t2578, t2579, t2582)
}
