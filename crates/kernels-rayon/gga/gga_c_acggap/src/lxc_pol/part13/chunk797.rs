//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 797/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk797(t1524: f64, t604: f64, t142: f64, t2060: f64, t1314: f64, t7815: f64, t7450: f64, t1318: f64, t2030: f64, t1545: f64, t7561: f64, t1549: f64, t7822: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8630 = t604 * t1524;
    let t8631 = t142 * t8630;
    let t8632 = t2060 * t8631;
    let t8634 = t7815 * t1314;
    let t8635 = t7450 * t8634;
    let t8637 = t7815 * t1318;
    let t8638 = t2030 * t8637;
    let t8640 = t7561 * t1545;
    let t8642 = t7822 * t1549;
    (t8630, t8631, t8632, t8634, t8635, t8637, t8638, t8640, t8642)
}
