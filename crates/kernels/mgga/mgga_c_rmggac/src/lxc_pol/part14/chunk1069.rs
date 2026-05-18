//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1069/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1069<F: Float>(t7255: F, t9165: F, t1652: F, t1970: F, t1971: F, t209: F, t476: F, t515: F, t7244: F, t8432: F, t1364: F, t1668: F, t41088: F, t42050: F, t42055: F, t42057: F, t42059: F, t42066: F, t42068: F, t42071: F, t42076: F, t42081: F, t42083: F, t42087: F, t42091: F, t6355: F, t7775: F, t7894: F) -> F {
    let t42093 = t7255 * t9165;
    let t42099 = t1970 * t1971 * t515 * t1652 * t476 * t209;
    let t42101 = t7244 * t8432;
    let t42103 = -F::new(0.25538759935978703638e-4) * t42050 + F::new(0.25538759935978703638e-4) * t42055 + F::new(0.43905552906833964735e0) * t42057 + F::new(0.2993560425465952141e-1) * t42059 - F::new(0.4726e1) * t1668 * t7894 - F::new(0.23948483403727617128e0) * t1364 * t41088 + F::new(0.8980681276397856423e-1) * t42066 - F::new(0.35922725105591425692e0) * t42068 - F::new(0.17961362552795712846e0) * t42071 + F::new(0.95770349759920138643e-4) * t42076 - F::new(0.11974241701863808564e0) * t6355 * t7775 - F::new(0.2993560425465952141e-1) * t42081 + F::new(0.1064114997332445985e-4) * t42083 + t42087 + F::new(0.42564599893297839398e-5) * t42091 + F::new(0.85129199786595678796e-5) * t42093 + F::new(0.85129199786595678796e-5) * t42099 - F::new(0.59590439850616975157e-4) * t42101;
    t42103
}
