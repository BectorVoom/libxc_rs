//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 783/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk783<F: Float>(t10061: F, t2782: F, t10059: F, t4086: F, t543: F, t123: F, t212: F, t2434: F, t4089: F, t138: F, t2438: F, t785: F) -> (F, F, F, F, F) {
    let t10062 = t2782 * t10061;
    let t10065 = t4086 * t10059 * t543;
    let t10066 = t2782 * t10065;
    let t10069 = t123 * t2434 * t212;
    let t10070 = t10069 * t4089;
    let t10073 = t138 * t2438 * t785;
    (t10062, t10066, t10069, t10070, t10073)
}
