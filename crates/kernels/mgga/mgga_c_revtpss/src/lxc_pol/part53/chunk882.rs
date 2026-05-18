//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 882/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk882<F: Float>(t25411: F, t27186: F, t213: F, t7759: F, t25431: F, t212: F, t780: F, t689: F, t1032: F, t1568: F, t1955: F, t7760: F, t786: F) -> (F, F, F, F, F, F, F) {
    let t27187 = t25411 * t27186;
    let t27189 = t213 * t7759;
    let t27192 = t25431 * t27186;
    let t27194 = t212 * t7759;
    let t27195 = t27194 * t780;
    let t27196 = t689 * t27195;
    let t27198 = t1568 * t1032;
    let t27199 = t1955 * t27198;
    let t27202 = t786 * t7760;
    (t27187, t27189, t27192, t27196, t27198, t27199, t27202)
}
