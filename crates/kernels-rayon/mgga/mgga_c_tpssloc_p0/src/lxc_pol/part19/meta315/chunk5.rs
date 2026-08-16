//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1126/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1126(t12083: f64, t172: f64, t763: f64, t12451: f64, t12466: f64, t12477: f64, t3734: f64, t39388: f64, t39393: f64, t39397: f64, t39400: f64, t39408: f64, t39411: f64, t39456: f64, t39463: f64, t39468: f64, t39472: f64, t39476: f64, t5126: f64, t5160: f64, t6999: f64) -> (f64, f64) {
    let t39478 = t12083 * t172 * t763;
    let t39479 = 0.23392894490538584828e1_f64 * t39478;
    let t39480 = -4.0_f64 * t12451 * t5160 * t6999 + 36.0_f64 * t12466 * t3734 * t5126 - 36.0_f64 * t12477 * t3734 * t5126 - t39388 + t39393 - t39397 - t39400 + t39408 + t39411 + t39456 + t39463 - t39468 - t39472 - t39476 - t39479;
    (t39479, t39480)
}
