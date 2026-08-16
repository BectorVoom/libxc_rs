//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 815/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk815(t21719: f64, t35155: f64, t9217: f64, t7248: f64, t9105: f64, t9110: f64, t15231: f64, t68432: f64, t68386: f64, t9117: f64, t9188: f64, t21708: f64, t21714: f64, t9183: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t74687 = t21719 * t35155 * t9217;
    let t74690 = t21719 * t7248 * t9105;
    let t74693 = t21719 * t7248 * t9110;
    let t74695 = t68432 * t15231;
    let t74698 = t68386 * t9188 * t9117;
    let t74701 = t21708 * t21714 * t9183;
    (t74687, t74690, t74693, t74695, t74698, t74701)
}
