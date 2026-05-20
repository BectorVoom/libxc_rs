//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1933/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1933<F: Float>(t1949: F, t231: F, t6016: F, t7076: F, t1558: F, t1579: F, t25392: F, t5977: F, t2723: F, t25416: F, t1955: F, t6041: F) -> (F, F, F, F, F, F, F, F, F) {
    let t29674 = t1949 * t6016 * t231;
    let t29675 = t7076 * t29674;
    let t29682 = t1579 * t1558 * t231;
    let t29683 = t25392 * t29682;
    let t29689 = t1949 * t5977;
    let t29690 = t29689 * t231;
    let t29691 = t7076 * t29690;
    let t29694 = t29689 * t2723;
    let t29695 = t25416 * t29694;
    let t29698 = t1955 * t6041;
    (t29674, t29675, t29682, t29683, t29690, t29691, t29694, t29695, t29698)
}
