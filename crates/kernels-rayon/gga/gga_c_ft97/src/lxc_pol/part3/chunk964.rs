//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 964/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk964(t2253: f64, t5450: f64, t5454: f64, t10845: f64, t4965: f64, t904: f64, t17766: f64, t4334: f64, t14487: f64, t17749: f64, t17753: f64, t2938: f64, t5468: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t18900 = t2253 * t5450;
    let t18902 = t2253 * t5454;
    let t18905 = t10845 * t4965 * t904;
    let t18908 = t4334 * t17766;
    let t18911 = t14487 * t17749;
    let t18914 = t4334 * t17753;
    let t18917 = t2938 * t5468;
    (t18900, t18902, t18905, t18908, t18911, t18914, t18917)
}
