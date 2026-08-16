//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 516/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk516(t1160: f64, t3062: f64, t180: f64, t879: f64, t407: f64, t1236: f64, t930: f64, t1529: f64, t315: f64) -> (f64, f64, f64, f64) {
    let t3063 = t1160 * t3062;
    let t3065 = t180 * t879;
    let t3066 = t3065 * t407;
    let t3067 = t1160 * t3066;
    let t3070 = t1236 * t930;
    let t3071 = t1160 * t3070;
    let t3073 = t315 * t1529;
    (t3063, t3067, t3071, t3073)
}
