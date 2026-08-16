//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2461/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2461(t12004: f64, t3111: f64, t1011: f64, t11165: f64, t15987: f64, t11156: f64, t15993: f64, t11692: f64, t11922: f64, t4899: f64, t1086: f64, t11213: f64, t3090: f64) -> (f64, f64, f64, f64, f64) {
    let t43019 = t12004 * t3111;
    let t43029 = t1011 * t15987 * t11165;
    let t43032 = t1011 * t15993 * t11156;
    let t43035 = t4899 * t11922 * t11692;
    let t43038 = t11213 * t1086 * t3090;
    (t43019, t43029, t43032, t43035, t43038)
}
