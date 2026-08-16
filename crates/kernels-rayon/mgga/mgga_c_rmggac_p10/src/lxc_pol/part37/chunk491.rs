//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 491/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk491(t13966: f64, t2040: f64, t2046: f64, t3167: f64, t7508: f64, t209: f64, t476: f64, t664: f64, t515: f64, t1971: f64, t1970: f64, t2164: f64, t668: f64) -> (f64, f64, f64, f64, f64) {
    let t13968 = t2046 * t13966 * t2040;
    let t13970 = t7508 * t3167;
    let t13973 = t664 * t476 * t209;
    let t13974 = t515 * t13973;
    let t13975 = t1971 * t13974;
    let t13976 = t1970 * t13975;
    let t13980 = t2164 * t668;
    (t13968, t13970, t13975, t13976, t13980)
}
