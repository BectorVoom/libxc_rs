//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 961/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk961(t4181: f64, t4801: f64, t1042: f64, t2852: f64, t3181: f64, t1592: f64, t3109: f64, t247: f64, t1063: f64, t1670: f64, t3172: f64, t1041: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4802 = t4801 * t4181;
    let t4803 = t1042 * t4802;
    let t4806 = t3181 * t2852;
    let t4807 = t4806 * t4181;
    let t4808 = t1042 * t4807;
    let t4816 = t3109 * t1592;
    let t4817 = t247 * t4816;
    let t4818 = t1063 * t4817;
    let t4820 = t3172 * t1670;
    let t4821 = t1041 * t4820;
    (t4802, t4803, t4806, t4807, t4808, t4817, t4818, t4820, t4821)
}
