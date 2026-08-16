//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 797/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk797(t1949: f64, t5844: f64, t5838: f64, t1599: f64, t7614: f64, t23678: f64, t5928: f64, t23677: f64, t23604: f64, t23603: f64, t28596: f64, t3188: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t28657 = t5844 * t1949;
    let t28660 = t5838 * t1949;
    let t28663 = t1599 * t7614;
    let t28666 = t5928 * t23678;
    let t28667 = t23677 * t28666;
    let t28670 = t5928 * t23604;
    let t28671 = t23603 * t28670;
    let t28674 = t28596 * t3188;
    (t28657, t28660, t28663, t28667, t28671, t28674)
}
