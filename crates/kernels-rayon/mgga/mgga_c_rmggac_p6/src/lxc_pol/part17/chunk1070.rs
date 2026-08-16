//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1070/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1070(t132: f64, t1811: f64, t7933: f64, t7934: f64, t575: f64, t577: f64, t3351: f64, t511: f64, t6403: f64, t9188: f64, t47124: f64, t515: f64) -> (f64, f64, f64, f64) {
    let t47549 = t7933 * t7934 * t1811 * t132;
    let t47553 = t7933 * t7934 * t577 * t575;
    let t47557 = t3351 * t9188 * t511 * t6403;
    let t47561 = t3351 * t9188 * t515 * t47124;
    (t47549, t47553, t47557, t47561)
}
