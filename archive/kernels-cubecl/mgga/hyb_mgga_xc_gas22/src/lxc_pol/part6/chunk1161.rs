//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1161/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1161<F: Float>(t10: F, t6291: F, t2034: F, t6469: F, t684: F, t3150: F, t689: F, t2038: F, t3023: F, t704: F, t2066: F, t6012: F) -> (F, F, F, F, F, F) {
    let t20241 = t6291 * t10;
    let t20252 = t684 * t6469 * t2034;
    let t20255 = t684 * t3150 * t689;
    let t20258 = t684 * t6469 * t2038;
    let t20278 = t3023 * t704;
    let t20280 = t6012 * t2066;
    (t20241, t20252, t20255, t20258, t20278, t20280)
}
