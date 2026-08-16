//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1048/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1048(t6355: f64, t9000: f64, t498: f64, t511: f64, t7230: f64, t7231: f64, t9969: f64, t321: f64, t3352: f64, t45347: f64, t674: f64, t2004: f64) -> (f64, f64, f64, f64, f64) {
    let t47835 = t6355 * t9000;
    let t47840 = t7230 * t7231 * t511 * t9969 * t498;
    let t47845 = t7230 * t3352 * t511 * t9969 * t321;
    let t47854 = t45347 * t674;
    let t47855 = t47854 * t2004;
    (t47835, t47840, t47845, t47854, t47855)
}
