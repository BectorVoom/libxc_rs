//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1204/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1204<F: Float>(t2873: F, t512: F, t524: F, t521: F, t536: F, t509: F, t523: F, t2880: F, t2938: F, t526: F, t7572: F, t527: F) -> (F, F, F, F, F, F, F) {
    let t22639 = F::new(1.0) / t2873 / t512;
    let t22640 = t524 * t22639;
    let t22645 = t536 * t521;
    let t22652 = t523 * t509;
    let t22653 = t22652 * t521;
    let t22662 = t2938 * t2880;
    let t22703 = t7572 * t526;
    let t22705 = F::new(1.0) / t527 / t22703;
    (t22639, t22640, t22645, t22653, t22662, t22703, t22705)
}
