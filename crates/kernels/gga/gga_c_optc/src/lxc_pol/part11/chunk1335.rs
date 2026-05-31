//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1335/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1335<F: Float>(t4835: F, t4846: F, t39411: F, t39413: F, t39418: F, t49240: F, t49242: F, t49271: F, t49273: F, t56966: F, t56969: F, t56972: F, t56975: F) -> (F, F, F) {
    let t58109 = t4835 * t4835;
    let t58115 = t4846 * t4846;
    let t58132 = -F::cast_from(0.19384444444444444445e4_f64) * t39411 - F::cast_from(0.12922962962962962963e4_f64) * t39413 + F::cast_from(0.38768888888888888889e4_f64) * t39418 + F::cast_from(0.19384444444444444445e4_f64) * t49240 - F::cast_from(0.58153333333333333333e4_f64) * t49242 - F::cast_from(0.12586666666666666667e4_f64) * t49271 + F::cast_from(0.20977777777777777778e3_f64) * t49273 + F::cast_from(17446.0_f64) * t56966 - F::cast_from(0.4846111111111111111e4_f64) * t56969 - F::cast_from(0.10488888888888888889e3_f64) * t56972 - F::cast_from(0.20977777777777777778e3_f64) * t56975;
    (t58109, t58115, t58132)
}
