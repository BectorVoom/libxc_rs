//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1354/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1354<F: Float>(t2760: F, t2783: F, t786: F, t2801: F, t10069: F, t10920: F, t231: F, t2782: F, t39709: F, t10910: F, t233: F, t689: F, t869: F) -> (F, F, F, F) {
    let t40297 = t786 * t2783 * t2760;
    let t40298 = t40297 * t2801;
    let t40303 = t10069 * t10920;
    let t40307 = t2782 * t2783 * t39709 * t231;
    let t40311 = t689 * t869 * t233 * t10910;
    (t40298, t40303, t40307, t40311)
}
