//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 957/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk957(t3645: f64, t547: f64, t1603: f64, t862: f64, t865: f64, t1659: f64, t3892: f64, t3035: f64, t3923: f64, t545: f64, t1658: f64, t316: f64, t449: f64, t879: f64) -> (f64, f64, f64, f64, f64) {
    let t15253 = t3645 * t547;
    let t15259 = t862 * t1603 * t865;
    let t15262 = t3892 * t1659;
    let t15265 = t3035 * t545 * t3923;
    let t15276 = t316 * t449 * t1658 * t879;
    (t15253, t15259, t15262, t15265, t15276)
}
