//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 215/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk215(t710: f64, t35: f64, t39: f64, t88: f64, t223: f64, t228: f64, t4: f64, t6: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t711 = 20.0_f64 * t710;
    let t712 = t35 * t39;
    let t713 = t712 * t88;
    let t714 = 12.0_f64 * t713;
    let t715 = t223 * t228;
    let t716 = t715 * t88;
    let t717 = 32.0_f64 * t716;
    let t721 = t4 * t6;
    (t711, t712, t714, t715, t717, t721)
}
