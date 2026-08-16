//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 589/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk589(t50: f64, t1702: f64, t829: f64, t1289: f64, t238: f64, t296: f64, t5468: f64, t5493: f64, t822: f64, t5492: f64, zeta_threshold: f64) -> f64 {
    let t51 = t50 <= zeta_threshold;
    let t5498 = t829 * t1702;
    let t5504 = piecewise3(t51, 0.0_f64, 8.0_f64 / 27.0_f64 * t5493 * t238 + 8.0_f64 / 9.0_f64 * t1289 * t822 - 2.0_f64 / 9.0_f64 * t5498 * t238 + 2.0_f64 / 3.0_f64 * t296 * t5468);
    let t5506 = t5492 / 2.0_f64 + t5504 / 2.0_f64;
    t5506
}
