//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 79/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk79(t205: f64, t207: f64, t211: f64, t216: f64, t30: f64) -> (f64, f64) {
    let t218 = -0.632975e0_f64 * t205 - 0.29896666666666666667e0_f64 * t207 - 0.1023875e0_f64 * t211 - 0.82156666666666666667e-1_f64 * t216;
    let t219 = 1.0_f64 / t30;
    (t218, t219)
}
