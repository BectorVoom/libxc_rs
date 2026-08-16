//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 888/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk888(t1587: f64, t236: f64, t3352: f64, t615: f64, t7230: f64, t10044: f64, t1982: f64, t7428: f64, t8365: f64, t8562: f64, t131: f64, t6344: f64, t638: f64, t639: f64, t71: f64) -> (f64, f64, f64, f64) {
    let t44906 = t7230 * t3352 * t236 * t1587 * t615;
    let t44909 = t10044 * t7428 * t1982;
    let t44911 = t8365 * t8562;
    let t44916 = t638 * t639 * t71 * t6344 * t131;
    (t44906, t44909, t44911, t44916)
}
