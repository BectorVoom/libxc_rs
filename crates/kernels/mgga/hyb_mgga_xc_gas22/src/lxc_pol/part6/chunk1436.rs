//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1436/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1436<F: Float>(t4851: F, t9488: F, t9625: F, t9778: F, t11589: F, t11594: F, t22600: F, t26103: F, t26122: F, t31126: F, t4502: F, t4505: F, t4513: F, t7602: F, t9444: F, t9458: F, t9474: F, t9598: F, t9612: F, t9624: F, t9737: F, t9742: F) -> F {
    let t31179 = t9488 * t4851;
    let t31197 = t9625 * t9778;
    let t31205 = -F::cast_from(11200.0_f64) / F::cast_from(9.0_f64) * t11589 * t9458 - F::cast_from(3200.0_f64) / F::cast_from(27.0_f64) * t9598 * t31179 + F::cast_from(3200.0_f64) / F::cast_from(27.0_f64) * t11594 * t9458 - F::cast_from(48.0_f64) * t26103 * t31126 - F::cast_from(720.0_f64) * t26122 * t9625 * t9444 - F::cast_from(64.0_f64) / F::cast_from(81.0_f64) * t22600 * t4502 - F::cast_from(3200.0_f64) / F::cast_from(3.0_f64) * t9612 * t31179 + F::cast_from(88.0_f64) / F::cast_from(27.0_f64) * t7602 * t4513 - F::cast_from(32.0_f64) / F::cast_from(27.0_f64) * t7602 * t4505 + F::cast_from(12.0_f64) * t9742 * t31197 - F::cast_from(180.0_f64) * t9624 * t9625 * t9474 + F::cast_from(252.0_f64) * t9737 * t31197;
    t31205
}
