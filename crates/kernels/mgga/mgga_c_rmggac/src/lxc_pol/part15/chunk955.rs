//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 955/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk955<F: Float>(t10050: F, t34857: F, t1987: F, t47854: F, t1990: F, t1979: F, t1982: F, t458: F, t9774: F, t38530: F, t8422: F, t42167: F, t42170: F, t42174: F, t42178: F, t42181: F, t47966: F, t47968: F, t47970: F, t47972: F, t47974: F, t47976: F, t47980: F, t47984: F) -> (F,) {
    let t47986 = t34857 * t10050;
    let t47988 = t47854 * t1987;
    let t47990 = t47854 * t1990;
    let t47994 = t9774 * t458 * t1979 * t1982;
    let t47996 = t38530 * t8422;
    let t47998 = 0.25538759935978703639e-4 * t47966 + 0.85129199786595678796e-5 * t47968 - 0.17961362552795712846e0 * t47970 + 0.35922725105591425692e0 * t47972 + 0.17961362552795712846e0 * t47974 + 0.79828278012425390427e-1 * t47976 - t42167 - 0.14408463291498358381e-2 * t42170 + 0.20496175532535769484e-3 * t42174 - t42178 - t42181 - 0.79828278012425390427e-1 * t47980 + 0.23942587439980034662e-4 * t47984 + 0.11971293719990017331e-4 * t47986 - 0.12769379967989351819e-4 * t47988 - 0.42564599893297839398e-5 * t47990 + 0.42564599893297839398e-5 * t47994 + 0.85129199786595678796e-5 * t47996;
    (t47998,)
}
