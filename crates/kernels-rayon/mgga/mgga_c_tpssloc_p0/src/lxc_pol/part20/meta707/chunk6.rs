//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2704/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2704(t1307: f64, t3698: f64, t1390: f64, t16486: f64, t16497: f64, t3734: f64, t3918: f64, t39384: f64, t39393: f64, t39397: f64, t39400: f64, t39408: f64, t39411: f64, t5126: f64, t54385: f64, t54388: f64, t54390: f64) -> (f64, f64) {
    let t55183 = t3698 * t1307;
    let t55191 = t16486 * t1390;
    let t55195 = 9.0_f64 * t1307 * t3918 * t55191 + 18.0_f64 * t16497 * t3734 * t5126 - t39384 + t39393 - t39397 - t39400 + t39408 + t39411 - t54385 - t54388 - t54390;
    (t55183, t55195)
}
