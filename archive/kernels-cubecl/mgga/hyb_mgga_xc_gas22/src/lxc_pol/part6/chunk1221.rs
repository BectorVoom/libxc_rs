//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1221/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1221<F: Float>(t1243: F, t3150: F, t684: F, t2014: F, t8493: F, t3161: F, t6469: F, t8465: F, t2024: F, t6479: F, t8469: F, t8481: F) -> (F, F, F, F, F, F) {
    let t23814 = t684 * t3150 * t1243;
    let t23817 = t684 * t2014 * t8493;
    let t23828 = t684 * t6469 * t3161;
    let t23831 = t684 * t2014 * t8465;
    let t23834 = t2024 * t6479 * t8469;
    let t23853 = t684 * t2014 * t8481;
    (t23814, t23817, t23828, t23831, t23834, t23853)
}
