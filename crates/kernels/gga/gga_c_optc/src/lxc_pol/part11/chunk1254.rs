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
    let t56717 = -F::new(0.30524261601532767229e2) * t7449 * t40356 * t50937 + t49816 / F::new(54.0) + F::new(0.48838818562452427568e2) * t49822 - t49833 / F::new(27.0) - t862 * t322 * t56700 / F::new(12.0) + t862 * t322 * t56704 / F::new(72.0) + t862 * t322 * t56708 / F::new(6.0) + F::new(5.0) / F::new(972.0) * t31479 - F::new(0.12209704640613106892e2) * t40328 + F::new(7.0) / F::new(486.0) * t49860 + t49865 / F::new(216.0) - F::new(0.24419409281226213784e2) * t49869;
    (t56708, t56717)
}
