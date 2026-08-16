//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 746/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk746(t15681: f64, t3020: f64, t384: f64, t938: f64, t4449: f64, t6: f64, t1620: f64, t4467: f64, t408: f64, t4491: f64, t11247: f64, t11361: f64, t15674: f64, t15677: f64, t15680: f64, t1617: f64, t1624: f64, t1669: f64, t3076: f64, t3077: f64, t3099: f64, t372: f64, t399: f64, t401: f64, t4476: f64, t4493: f64, t8009: f64, t8015: f64) -> (f64, f64, f64) {
    let t15682 = t3020 * t15681;
    let t15689 = t384 * t938;
    let t15693 = t4449 * t6;
    let t15694 = t15693 * t1620;
    let t15697 = t4467 * t6;
    let t15706 = t408 * t4491;
    let t15710 = 0.11627450473218896e-1_f64 * t1624 * t15674 + 0.23254900946437792e-2_f64 * t372 * t15677 - 0.33776098467676728323e-5_f64 * t15680 * t15682 - 0.59273806478425129876e-2_f64 * t4493 * t399 + 0.11854761295685025975e-1_f64 * t4476 * t399 - 0.38731446812548799881e-3_f64 * t11361 * t11247 * t15689 + 0.13784064983740990796e-4_f64 * t8015 * t15694 - 0.68920324918704953981e-4_f64 * t1617 * t15697 * t1620 + 0.45915205659928668026e-5_f64 * t8009 * t15694 + 4.0_f64 * t3076 * t3077 * t3099 - 2.0_f64 * t1669 * t15706 * t401;
    (t15682, t15689, t15710)
}
