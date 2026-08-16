//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1335/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1335(t4835: f64, t4846: f64, t39411: f64, t39413: f64, t39418: f64, t49240: f64, t49242: f64, t49271: f64, t49273: f64, t56966: f64, t56969: f64, t56972: f64, t56975: f64) -> (f64, f64, f64) {
    let t58109 = t4835 * t4835;
    let t58115 = t4846 * t4846;
    let t58132 = -0.19384444444444444445e4_f64 * t39411 - 0.12922962962962962963e4_f64 * t39413 + 0.38768888888888888889e4_f64 * t39418 + 0.19384444444444444445e4_f64 * t49240 - 0.58153333333333333333e4_f64 * t49242 - 0.12586666666666666667e4_f64 * t49271 + 0.20977777777777777778e3_f64 * t49273 + 17446.0_f64 * t56966 - 0.4846111111111111111e4_f64 * t56969 - 0.10488888888888888889e3_f64 * t56972 - 0.20977777777777777778e3_f64 * t56975;
    (t58109, t58115, t58132)
}
