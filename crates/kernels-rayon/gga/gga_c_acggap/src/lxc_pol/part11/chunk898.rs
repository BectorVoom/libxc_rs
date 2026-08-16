//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 898/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk898(t1983: f64, t30692: f64, t3196: f64, t7586: f64, t1530: f64, t7336: f64, t1992: f64, t7842: f64, t30148: f64, t3176: f64, t7585: f64, t174: f64, t30423: f64, t3126: f64, t3157: f64, t7323: f64) -> (f64, f64, f64, f64, f64) {
    let t30695 = t30692 * t7586 * t1983 * t3196;
    let t30698 = t1530 * t7336;
    let t30705 = t30692 * t7842 * t1992 * t3196;
    let t30709 = t7585 * t7842 * t30148 * t3176;
    let t30714 = t30423 * t7323 * t174 * t3157 * t3126;
    (t30695, t30698, t30705, t30709, t30714)
}
