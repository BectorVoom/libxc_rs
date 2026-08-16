//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 814/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk814(t495: f64, t560: f64, t2541: f64, t1734: f64, t469: f64, t1814: f64, t609: f64, t944: f64, t7890: f64, t1914: f64, t8004: f64, t157: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9476 = t495 * t560;
    let t9477 = t2541 * t9476;
    let t9480 = t469 * t1734;
    let t9491 = t609 * t1814;
    let t9492 = t9491 * t944;
    let t9493 = t7890 * t9492;
    let t9497 = t609 * t1914;
    let t9498 = t8004 * t9497;
    let t9502 = t9491 * t157;
    (t9476, t9477, t9480, t9493, t9497, t9498, t9502)
}
