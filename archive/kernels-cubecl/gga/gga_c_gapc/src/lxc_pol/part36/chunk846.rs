//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 846/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk846<F: Float>(t10133: F, t3192: F, t3206: F, t763: F, t462: F, t762: F, t3193: F, t126: F, t818: F, t787: F, t3187: F, t297: F, t3727: F) -> (F, F, F, F, F, F) {
    let t10134 = t3192 * t10133;
    let t10137 = t763 * t3206;
    let t10139 = t462 * t762;
    let t10140 = t10139 * t3193;
    let t10142 = t126 * t818;
    let t10143 = t10142 * t787;
    let t10144 = t3187 * t10143;
    let t10146 = t3727 * t297;
    (t10134, t10137, t10140, t10142, t10144, t10146)
}
