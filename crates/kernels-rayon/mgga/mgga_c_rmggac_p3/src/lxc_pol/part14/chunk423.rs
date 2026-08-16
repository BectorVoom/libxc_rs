//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 423/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk423(t1003: f64, t230: f64, t1001: f64, t195: f64, t1131: f64, t388: f64, t155: f64, t1041: f64, t971: f64, t416: f64, t171: f64, t4157: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4179 = 1.0_f64 / t1003 / t230;
    let t4182 = t195 * t1001;
    let t4186 = t388 * t1131;
    let t4187 = t155 * t4186;
    let t4189 = t971 * t1041;
    let t4190 = t4189 * t416;
    let t4202 = t171 * t4157;
    (t4179, t4182, t4187, t4189, t4190, t4202)
}
