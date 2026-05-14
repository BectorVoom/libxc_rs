//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1041/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1041<F: Float>(t212: F, t7759: F, t780: F, t689: F, t1032: F, t1568: F, t1955: F) -> (F, F, F, F, F) {
    let t27194 = t212 * t7759;
    let t27195 = t27194 * t780;
    let t27196 = t689 * t27195;
    let t27198 = t1568 * t1032;
    let t27199 = t1955 * t27198;
    (t27194, t27195, t27196, t27198, t27199)
}
