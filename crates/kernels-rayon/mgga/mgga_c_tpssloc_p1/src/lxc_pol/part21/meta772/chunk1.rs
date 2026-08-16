//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2674/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2674(t54387: f64, t54389: f64, t19575: f64, t592: f64, t15904: f64, t16486: f64, t16497: f64, t1845: f64, t193: f64, t19603: f64, t33159: f64, t39393: f64, t39397: f64, t39400: f64, t39408: f64, t39411: f64, t5126: f64, t5160: f64, t5161: f64, t5308: f64, t531: f64, t55224: f64) -> (f64, f64, f64, f64) {
    let t56178 = 0.11696447245269292414e1_f64 * t54387;
    let t56179 = 0.11696447245269292414e1_f64 * t54389;
    let t56185 = t592 * t19575;
    let t56186 = 8.0_f64 * t56185;
    let t56192 = -24.0_f64 * t15904 * t1845 * t193 * t33159 * t531 - 2.0_f64 * t16486 * t5160 * t5161 + 24.0_f64 * t16497 * t5126 * t5308 + 24.0_f64 * t19603 * t55224 + t39393 - t39397 - t39400 + t39408 + t39411 - t56178 - t56179 - t56186;
    (t56178, t56179, t56186, t56192)
}
