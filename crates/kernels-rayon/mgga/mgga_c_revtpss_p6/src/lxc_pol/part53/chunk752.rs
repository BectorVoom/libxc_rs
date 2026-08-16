//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 752/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk752(t1936: f64, t4248: f64, t1518: f64, t93: f64, t1312: f64, t7741: f64, t6985: f64, t7725: f64, t1847: f64, t196: f64, t197: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7888 = 2.0_f64 * t4248 * t1936;
    let t7889 = t93 * t1518;
    let t7891 = 2.0_f64 * t7889 * t1936;
    let t7893 = 2.0_f64 * t1312 * t7741;
    let t7894 = 2.0_f64 * t1518 * t6985 + t7725 + t7888 + t7891 + t7893;
    let t7897 = t1847 * t196;
    let t7898 = t7897 * t197;
    (t7888, t7889, t7891, t7893, t7894, t7898)
}
