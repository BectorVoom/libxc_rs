//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 914/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk914(t1543: f64, t797: f64, t2259: f64, t2330: f64, t6897: f64, t1234: f64, t3264: f64, t792: f64, t1103: f64, t1783: f64, t1053: f64, t1102: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10611 = t797 * t1543;
    let t10622 = t797 * t2259;
    let t10626 = t6897 * t2330;
    let t10630 = t797 * t1234;
    let t10634 = t3264 * t792;
    let t10641 = t1103 * t1783;
    let t10643 = t1102 * t1053 * t10641;
    (t10611, t10622, t10626, t10630, t10634, t10641, t10643)
}
