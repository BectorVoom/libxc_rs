//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 26/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk26(t11: f64, t14: f64, t17: f64, t25: f64) -> (f64, f64, f64, f64) {
    let t67 = 0.705945e1_f64 * t14 + 0.1549425e1_f64 * t11 + 0.420775e0_f64 * t17 + 0.1562925e0_f64 * t25;
    let t70 = 1.0_f64 + 0.32163958997385070134e2_f64 / t67;
    let t71 = f64::ln(t70);
    let t75 = 1.0_f64 + 0.278125e-1_f64 * t11;
    (t67, t70, t71, t75)
}
