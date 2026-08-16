//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3088/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3088(t11675: f64, t15682: f64, t11711: f64, t15618: f64, t1043: f64, t1469: f64, t3133: f64, t3162: f64, t3115: f64, t42793: f64, t4906: f64, t11722: f64, t4834: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t53559 = t11675 * t15682;
    let t53567 = t15618 * t11711;
    let t53585 = t1469 * t1043;
    let t53586 = t3162 * t3133;
    let t53612 = t3115 * t42793 * t4906;
    let t53626 = t4834 * t11722;
    (t53559, t53567, t53585, t53586, t53612, t53626)
}
