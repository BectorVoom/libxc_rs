//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 1101/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk1101(t38530: f64, t8422: f64, t42167: f64, t42170: f64, t42174: f64, t42178: f64, t42181: f64, t47966: f64, t47968: f64, t47970: f64, t47972: f64, t47974: f64, t47976: f64, t47980: f64, t47984: f64, t47986: f64, t47988: f64, t47990: f64, t47994: f64) -> f64 {
    let t47996 = t38530 * t8422;
    let t47998 = 0.25538759935978703639e-4_f64 * t47966 + 0.85129199786595678796e-5_f64 * t47968 - 0.17961362552795712846e0_f64 * t47970 + 0.35922725105591425692e0_f64 * t47972 + 0.17961362552795712846e0_f64 * t47974 + 0.79828278012425390427e-1_f64 * t47976 - t42167 - 0.14408463291498358381e-2_f64 * t42170 + 0.20496175532535769484e-3_f64 * t42174 - t42178 - t42181 - 0.79828278012425390427e-1_f64 * t47980 + 0.23942587439980034662e-4_f64 * t47984 + 0.11971293719990017331e-4_f64 * t47986 - 0.12769379967989351819e-4_f64 * t47988 - 0.42564599893297839398e-5_f64 * t47990 + 0.42564599893297839398e-5_f64 * t47994 + 0.85129199786595678796e-5_f64 * t47996;
    t47998
}
