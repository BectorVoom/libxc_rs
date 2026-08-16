//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2985/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2985(t1065: f64, t15648: f64, t15772: f64, t3188: f64, t1063: f64, t16195: f64, t3172: f64, t16200: f64, t15775: f64, t16204: f64, t16209: f64, t10326: f64, t1469: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t54419 = t1065 * t15648;
    let t54432 = t3188 * t15772;
    let t54435 = t1063 * t3172 * t16195;
    let t54438 = t1063 * t3172 * t16200;
    let t54440 = t3188 * t15775;
    let t54443 = t1063 * t3172 * t16204;
    let t54446 = t1063 * t3172 * t16209;
    let t54450 = t1469 * t10326;
    (t54419, t54432, t54435, t54438, t54440, t54443, t54446, t54450)
}
