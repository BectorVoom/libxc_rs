//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1007/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1007<F: Float>(t3008: F, t545: F, t9899: F, t1802: F, t3804: F, t3014: F, t8587: F, t8588: F) -> (F, F, F, F) {
    let t9901 = t3008 * t9899 * t545;
    let t9904 = t1802 * t3804;
    let t9906 = t3014 * t9904 * t545;
    let t9909 = -t8587 - t8588;
    (t9901, t9904, t9906, t9909)
}
