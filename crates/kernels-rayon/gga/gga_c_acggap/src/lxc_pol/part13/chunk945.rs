//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 945/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk945(t137: f64, t3037: f64, t1089: f64, t1095: f64, t31520: f64, t2113: f64, t7780: f64, t1967: f64, t7681: f64, t3652: f64, t7741: f64, t3657: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t31521 = t137 * t3037;
    let t31524 = t31520 * t1089 * t1095 * t31521;
    let t31525 = 0.94344276868812456204e-3_f64 * t31524;
    let t31526 = t7780 * t2113;
    let t31528 = t1967 * t7681;
    let t31530 = t7741 * t3652;
    let t31532 = t7741 * t3657;
    (t31521, t31525, t31526, t31528, t31530, t31532)
}
