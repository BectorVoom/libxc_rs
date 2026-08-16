//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1273/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1273(t4189: f64, t7397: f64, t7962: f64, t1505: f64, t29412: f64, t1555: f64, t22310: f64, t94833: f64, t29424: f64, t39301: f64, t22300: f64, t17311: f64, t28576: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t101826 = 2.0_f64 * t4189 * t7962 * t7397;
    let t101827 = t29412 * t1505;
    let t101828 = t101827 * t1555;
    let t101830 = 6.0_f64 * t94833 * t22310;
    let t101832 = 6.0_f64 * t39301 * t29424;
    let t101833 = t22300 * t7962;
    let t101835 = 4.0_f64 * t17311 * t28576;
    (t101826, t101828, t101830, t101832, t101833, t101835)
}
