//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1027/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1027<F: Float>(t10059: F, t4086: F, t543: F, t2782: F, t123: F, t212: F, t2434: F, t4089: F, t138: F, t2438: F, t785: F, t1398: F, t1419: F) -> (F, F, F, F, F, F) {
    let t10065 = t4086 * t10059 * t543;
    let t10066 = t2782 * t10065;
    let t10069 = t123 * t2434 * t212;
    let t10070 = t10069 * t4089;
    let t10073 = t138 * t2438 * t785;
    let t10074 = t10073 * t4089;
    let t10079 = t4086 * t1419 * t1398 * t543;
    (t10066, t10069, t10070, t10073, t10074, t10079)
}
