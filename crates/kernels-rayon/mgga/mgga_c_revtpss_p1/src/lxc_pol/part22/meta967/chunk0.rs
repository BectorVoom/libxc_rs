//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3231/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3231(t14622: f64, t18259: f64, t18281: f64, t189: f64, t4401: f64, t606: f64, t190: f64, t2611: f64, t60717: f64, t18555: f64, t2619: f64, t13396: f64, t14330: f64, t4402: f64) -> (f64, f64, f64, f64, f64) {
    let t61265 = 24.0_f64 * t18259 * t14622;
    let t61266 = t189 * t18281;
    let t61269 = 24.0_f64 * t4401 * t61266 * t606;
    let t61274 = 24.0_f64 * t2611 * t190 * t60717;
    let t61282 = t18555 * t2619;
    let t61283 = 0.24415263074675393405e-3_f64 * t61282;
    let t61286 = 96.0_f64 * t14330 * t4402 * t13396;
    (t61265, t61269, t61274, t61283, t61286)
}
