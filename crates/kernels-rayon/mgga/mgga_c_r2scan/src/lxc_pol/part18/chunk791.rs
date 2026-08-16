//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 791/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk791(t1842: f64, t963: f64, t1814: f64, t5249: f64, t897: f64, t5252: f64, t2743: f64, t5326: f64, t1419: f64, t959: f64, t2483: f64, t725: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7689 = t963 * t1842;
    let t7691 = t963 * t1814;
    let t7693 = t5249 * t897;
    let t7694 = t7693 * t5252;
    let t7699 = t2743 * t5326;
    let t7701 = t1419 * t959;
    let t7705 = t2483 * t725;
    (t7689, t7691, t7694, t7699, t7701, t7705)
}
