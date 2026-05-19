//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 1101/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk1101<F: Float>(t38530: F, t8422: F, t42167: F, t42170: F, t42174: F, t42178: F, t42181: F, t47966: F, t47968: F, t47970: F, t47972: F, t47974: F, t47976: F, t47980: F, t47984: F, t47986: F, t47988: F, t47990: F, t47994: F) -> F {
    let t47996 = t38530 * t8422;
    let t47998 = F::cast_from(0.25538759935978703639e-4_f64) * t47966 + F::cast_from(0.85129199786595678796e-5_f64) * t47968 - F::cast_from(0.17961362552795712846e0_f64) * t47970 + F::cast_from(0.35922725105591425692e0_f64) * t47972 + F::cast_from(0.17961362552795712846e0_f64) * t47974 + F::cast_from(0.79828278012425390427e-1_f64) * t47976 - t42167 - F::cast_from(0.14408463291498358381e-2_f64) * t42170 + F::cast_from(0.20496175532535769484e-3_f64) * t42174 - t42178 - t42181 - F::cast_from(0.79828278012425390427e-1_f64) * t47980 + F::cast_from(0.23942587439980034662e-4_f64) * t47984 + F::cast_from(0.11971293719990017331e-4_f64) * t47986 - F::cast_from(0.12769379967989351819e-4_f64) * t47988 - F::cast_from(0.42564599893297839398e-5_f64) * t47990 + F::cast_from(0.42564599893297839398e-5_f64) * t47994 + F::cast_from(0.85129199786595678796e-5_f64) * t47996;
    t47998
}
