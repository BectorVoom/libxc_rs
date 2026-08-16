//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 793/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk793(t1524: f64, t599: f64, t336: f64, t578: f64, t2020: f64, t515: f64, t604: f64, t142: f64, t2060: f64, t1314: f64, t7815: f64, t7450: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8621 = t599 * t1524;
    let t8622 = t336 * t8621;
    let t8623 = t578 * t8622;
    let t8625 = t2020 * t515;
    let t8630 = t604 * t1524;
    let t8631 = t142 * t8630;
    let t8632 = t2060 * t8631;
    let t8634 = t7815 * t1314;
    let t8635 = t7450 * t8634;
    (t8622, t8623, t8625, t8630, t8631, t8632, t8634, t8635)
}
