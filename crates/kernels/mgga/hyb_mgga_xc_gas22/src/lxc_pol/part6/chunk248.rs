//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 248/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk248<F: Float>(t238: F, t243: F, t801: F, t226: F, t779: F, t242: F, t781: F, t792: F, t794: F, t797: F) -> (F, F, F, F, F) {
    let t803 = t238 * t801 * t243;
    let t804 = 0.82156666666666666667e-1 * t803;
    let t805 = t226 * t779;
    let t807 = t238 * t242 * t805;
    let t809 = 0.1898925e1 * t792 - t794 + 0.8969e0 * t781 + 0.3071625e0 * t797 - t804 + 0.24647e0 * t807;
    (t803, t804, t805, t807, t809)
}
