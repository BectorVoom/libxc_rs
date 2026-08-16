//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1095/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1095(t80351: f64, t80355: f64, t80366: f64, t80370: f64, t235: f64, t515: f64, t70063: f64, t70101: f64, t71670: f64, t71671: f64, t71672: f64, t78352: f64, t78355: f64, t78359: f64, t78362: f64, t78364: f64, t78368: f64, t78371: f64, t78372: f64, t78375: f64, t80347: f64, t80349: f64) -> (f64, f64) {
    let t80372 = t80351 + t80355 + t80366 + t80370;
    let t80376 = -t78352 - t78355 + t80347 - 0.91976356987732177729e-5_f64 * t70063 - t80349 - t71670 - t71671 - t71672 + t78359 - t70101 + t78362 + t78364 - t78368 - 0.19957069503106347607e-1_f64 * t235 * t515 * t80372 + t78371 - t78372 - t78375;
    (t80372, t80376)
}
