//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1265/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1265(t125428: f64, t2014: f64, t2107: f64, t102070: f64, t1448: f64, t28196: f64, t34297: f64, t25082: f64, t27153: f64, t37318: f64, t32738: f64, t98450: f64) -> (f64, f64, f64, f64) {
    let t128910 = t2014 * t2107 * t125428;
    let t128917 = 6.0_f64 * t28196 * t102070 * t34297 * t1448;
    let t128920 = 3.0_f64 * t25082 * t37318 * t27153;
    let t128930 = 3.0_f64 * t98450 * t32738;
    (t128910, t128917, t128920, t128930)
}
