//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 663/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk663<F: Float>(t2625: F, t506: F, t529: F, t2531: F, t538: F, t560: F, t938: F) -> (F, F, F) {
    let t2626 = t506 * t2625;
    let t2627 = t529 * t2626;
    let t2630 = t538 * t2531;
    let t2631 = t529 * t2630;
    let t2634 = t938 * t560;
    (t2627, t2631, t2634)
}
