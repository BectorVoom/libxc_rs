//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 999/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk999(t4249: f64, t7283: f64, t6002: f64, t6034: f64, t2066: f64, t6020: f64, t21955: f64, t577: f64, t1548: f64, t1929: f64, t570: f64, t5910: f64) -> (f64, f64, f64, f64, f64) {
    let t22685 = t4249 * t7283;
    let t22687 = t6002 * t6034;
    let t22689 = t6020 * t2066;
    let t22691 = t21955 * t577;
    let t22692 = t22691 * t1548;
    let t22694 = t570 * t1929;
    let t22695 = t22694 * t5910;
    (t22685, t22687, t22689, t22692, t22695)
}
