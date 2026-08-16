//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1013/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1013(t2014: f64, t32129: f64, t6972: f64, t8441: f64, t8621: f64, t1936: f64, t25805: f64, t28025: f64, t6985: f64, t7002: f64, t648: f64, t8453: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t32131 = 2.0_f64 * t2014 * t32129;
    let t32151 = t8621 * t8441 * t6972;
    let t32165 = t25805 * t1936;
    let t32167 = t28025 * t1936;
    let t32169 = t6985 * t7002;
    let t32171 = t648 * t8453;
    (t32131, t32151, t32165, t32167, t32169, t32171)
}
