//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3235/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3235<F: Float>(t2439: F, t6041: F, t780: F, t785: F, t4533: F, t18821: F, t2471: F, t18814: F, t2435: F, t14476: F, t1580: F, t689: F) -> (F, F, F, F, F) {
    let t61324 = t2439 * t785 * t6041 * t780;
    let t61326 = t4533 * t4533;
    let t61330 = t18821 * t2471;
    let t61337 = t2435 * t18814;
    let t61344 = t689 * t14476 * t1580;
    (t61324, t61326, t61330, t61337, t61344)
}
