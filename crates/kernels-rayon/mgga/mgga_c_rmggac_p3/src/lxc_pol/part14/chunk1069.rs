//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1069/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1069(t7255: f64, t9165: f64, t1652: f64, t1970: f64, t1971: f64, t209: f64, t476: f64, t515: f64, t7244: f64, t8432: f64, t1364: f64, t1668: f64, t41088: f64, t42050: f64, t42055: f64, t42057: f64, t42059: f64, t42066: f64, t42068: f64, t42071: f64, t42076: f64, t42081: f64, t42083: f64, t42087: f64, t42091: f64, t6355: f64, t7775: f64, t7894: f64) -> f64 {
    let t42093 = t7255 * t9165;
    let t42099 = t1970 * t1971 * t515 * t1652 * t476 * t209;
    let t42101 = t7244 * t8432;
    let t42103 = -0.25538759935978703638e-4_f64 * t42050 + 0.25538759935978703638e-4_f64 * t42055 + 0.43905552906833964735e0_f64 * t42057 + 0.2993560425465952141e-1_f64 * t42059 - 0.4726e1_f64 * t1668 * t7894 - 0.23948483403727617128e0_f64 * t1364 * t41088 + 0.8980681276397856423e-1_f64 * t42066 - 0.35922725105591425692e0_f64 * t42068 - 0.17961362552795712846e0_f64 * t42071 + 0.95770349759920138643e-4_f64 * t42076 - 0.11974241701863808564e0_f64 * t6355 * t7775 - 0.2993560425465952141e-1_f64 * t42081 + 0.1064114997332445985e-4_f64 * t42083 + t42087 + 0.42564599893297839398e-5_f64 * t42091 + 0.85129199786595678796e-5_f64 * t42093 + 0.85129199786595678796e-5_f64 * t42099 - 0.59590439850616975157e-4_f64 * t42101;
    t42103
}
