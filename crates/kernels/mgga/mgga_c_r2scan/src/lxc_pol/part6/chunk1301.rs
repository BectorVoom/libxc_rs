//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1301/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1301<F: Float>(t7494: F, t7520: F, t6118: F, t8039: F, t2598: F, t2599: F, t6848: F, t2294: F, t2582: F, t7527: F, t7934: F, t6132: F, t7538: F, t8034: F, t6106: F, t7379: F) -> (F, F, F, F, F, F, F, F) {
    let t24527 = t7494 * t7520;
    let t24543 = t6118 * t8039;
    let t24546 = t2598 * t6848 * t2599;
    let t24547 = 0.25426783770825854452e1 * t24546;
    let t24549 = t2582 * t2294 * t7527;
    let t24555 = t7494 * t7934;
    let t24558 = t6132 * t2294 * t7538;
    let t24565 = t2598 * t2294 * t8034;
    let t24568 = t6106 * t2294 * t7379;
    (t24527, t24543, t24547, t24549, t24555, t24558, t24565, t24568)
}
