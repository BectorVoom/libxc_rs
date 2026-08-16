//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 989/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk989(t30777: f64, t2290: f64, t7630: f64, t1549: f64, t30540: f64, t1554: f64, t1558: f64, t30137: f64, t7585: f64, t8525: f64, t1072: f64, t535: f64, t7507: f64, t7512: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t34848 = 0.17149607247227894789e-2_f64 * t30777;
    let t34849 = t7630 * t2290;
    let t34851 = t30540 * t1549;
    let t34853 = t30540 * t1554;
    let t34855 = t30540 * t1558;
    let t34865 = t7585 * t30137 * t8525;
    let t34879 = t7507 * t7512 * t535 * t1072;
    (t34848, t34849, t34851, t34853, t34855, t34865, t34879)
}
