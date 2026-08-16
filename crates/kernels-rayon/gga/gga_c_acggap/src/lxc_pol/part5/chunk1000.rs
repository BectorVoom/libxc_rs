//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1000/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1000(t3409: f64, t4414: f64, t14223: f64, t4376: f64, t14047: f64, t4904: f64, t1101: f64, t1165: f64, t1586: f64, t3361: f64, t16548: f64, t3194: f64, t530: f64) -> (f64, f64, f64, f64, f64) {
    let t16757 = t3409 * t4414;
    let t16759 = t14223 * t4376;
    let t16765 = t14047 * t4904;
    let t16769 = t3361 * t1165 * t1586 * t1101;
    let t16779 = t3194 * t1165 * t530 * t16548;
    (t16757, t16759, t16765, t16769, t16779)
}
