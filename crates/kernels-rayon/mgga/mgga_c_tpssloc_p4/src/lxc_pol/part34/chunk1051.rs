//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1051/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1051(t25224: f64, t7479: f64, t6552: f64, t23195: f64, t5636: f64, t6553: f64, t1880: f64, t5527: f64, t6554: f64, t23035: f64, t16815: f64, t232: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t28288 = t25224 * t7479;
    let t28289 = t6552 * t28288;
    let t28294 = t23195 * t5636;
    let t28295 = t6553 * t28294;
    let t28296 = t1880 * t28295;
    let t28298 = t6554 * t5527;
    let t28299 = t6553 * t28298;
    let t28300 = t23035 * t28299;
    let t28321 = t16815 * t232;
    (t28288, t28289, t28294, t28295, t28296, t28298, t28299, t28300, t28321)
}
