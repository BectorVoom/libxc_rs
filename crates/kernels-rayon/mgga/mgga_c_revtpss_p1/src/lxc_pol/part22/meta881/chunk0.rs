//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3053/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3053(t22: f64, t231: f64, t39698: f64, t4494: f64, t2782: f64, t2783: f64, t51375: f64, t10073: f64, t14509: f64, t10069: f64, t40921: f64, t4496: f64) -> (f64, f64, f64, f64, f64) {
    let t51676 = t39698 * t4494 * t231 * t22;
    let t51680 = t2782 * t2783 * t51375 * t231;
    let t51682 = t10073 * t14509;
    let t51684 = t10069 * t14509;
    let t51686 = t40921 * t4496;
    (t51676, t51680, t51682, t51684, t51686)
}
