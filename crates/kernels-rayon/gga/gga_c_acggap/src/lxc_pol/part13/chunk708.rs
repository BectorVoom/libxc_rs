//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 708/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk708(t137: f64, t864: f64, t1089: f64, t1095: f64, t7553: f64, t121: f64, t163: f64, t1171: f64) -> (f64, f64, f64, f64, f64) {
    let t7554 = t137 * t864;
    let t7556 = t1089 * t1095 * t7554;
    let t7557 = t7553 * t7556;
    let t7558 = 0.31448092289604152068e-3_f64 * t7557;
    let t7559 = t121 * t163;
    let t7560 = t7559 * t1171;
    (t7554, t7556, t7558, t7559, t7560)
}
