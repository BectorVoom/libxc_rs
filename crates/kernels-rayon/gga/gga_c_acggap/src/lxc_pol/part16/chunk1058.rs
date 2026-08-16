//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1058/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1058(t1165: f64, t5567: f64, t7426: f64, t8600: f64, t5572: f64, t7575: f64, t30120: f64, t9645: f64, t1815: f64, t1992: f64, t30127: f64, t7842: f64) -> (f64, f64, f64, f64) {
    let t38717 = t7426 * t1165 * t8600 * t5567;
    let t38721 = t7575 * t1165 * t8600 * t5572;
    let t38723 = t30120 * t9645;
    let t38727 = t30127 * t7842 * t1992 * t1815;
    (t38717, t38721, t38723, t38727)
}
