//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1254/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1254<F: Float>(t25174: F, t55901: F, t31479: F, t322: F, t40328: F, t40356: F, t49816: F, t49822: F, t49833: F, t49860: F, t49865: F, t49869: F, t50937: F, t56700: F, t56704: F, t7449: F, t862: F) -> (F, F) {
    let t56708 = t25174 * t55901;
    let t56717 = -F::cast_from(0.30524261601532767229e2_f64) * t7449 * t40356 * t50937 + t49816 / F::cast_from(54.0_f64) + F::cast_from(0.48838818562452427568e2_f64) * t49822 - t49833 / F::cast_from(27.0_f64) - t862 * t322 * t56700 / F::cast_from(12.0_f64) + t862 * t322 * t56704 / F::cast_from(72.0_f64) + t862 * t322 * t56708 / F::cast_from(6.0_f64) + F::cast_from(5.0_f64) / F::cast_from(972.0_f64) * t31479 - F::cast_from(0.12209704640613106892e2_f64) * t40328 + F::cast_from(7.0_f64) / F::cast_from(486.0_f64) * t49860 + t49865 / F::cast_from(216.0_f64) - F::cast_from(0.24419409281226213784e2_f64) * t49869;
    (t56708, t56717)
}
