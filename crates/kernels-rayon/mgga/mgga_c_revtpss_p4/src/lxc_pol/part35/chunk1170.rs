//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1170/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1170(t2411: f64, t30419: f64, t105936: f64, t95822: f64, t212: f64, t30379: f64, t689: f64, t780: f64, t95537: f64, t213: f64, t30410: f64, t686: f64, t72: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t110177 = t30419 * t2411;
    let t110236 = t95822 * t105936;
    let t110245 = t689 * t212 * t30379 * t780;
    let t110247 = t95537 * t105936;
    let t110256 = t213 * t30379;
    let t110275 = t30410 * t72 * t686;
    (t110177, t110236, t110245, t110247, t110256, t110275)
}
