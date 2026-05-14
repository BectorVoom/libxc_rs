//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 698/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk698<F: Float>(t10: F, t1523: F, t1107: F, t1057: F, t1524: F, t1052: F, t3636: F, t495: F) -> (F, F, F, F, F) {
    let t3639 = t1523 * t10;
    let t3640 = t3639 * t1107;
    let t3643 = t1057 * t1524;
    let t3645 = t1052 * t1524;
    let t3647 = t3636 * t495;
    (t3639, t3640, t3643, t3645, t3647)
}
