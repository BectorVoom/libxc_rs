//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3105/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3105(t1045: f64, t606: f64, t11937: f64, t15671: f64, t11262: f64, t3127: f64, t4824: f64, t1065: f64, t15648: f64, t15772: f64, t3188: f64, t1063: f64, t16195: f64, t3172: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t54397 = t1045 * t606;
    let t54407 = t15671 * t11937;
    let t54414 = t3127 * t11262 * t4824;
    let t54419 = t1065 * t15648;
    let t54432 = t3188 * t15772;
    let t54435 = t1063 * t3172 * t16195;
    (t54397, t54407, t54414, t54419, t54432, t54435)
}
