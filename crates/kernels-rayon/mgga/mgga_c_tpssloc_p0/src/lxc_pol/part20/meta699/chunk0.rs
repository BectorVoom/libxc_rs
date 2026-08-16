//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2666/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2666(t5154: f64, t9905: f64, t15968: f64, t67: f64, t758: f64, t17: f64, t750: f64, t2225: f64, t5166: f64, t15921: f64, t592: f64, t39478: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t54392 = t5154 * t9905;
    let t54393 = 0.35089341735807877242e1_f64 * t54392;
    let t54395 = t15968 * t67 * t758;
    let t54396 = 0.54934341918019635162e-3_f64 * t54395;
    let t54398 = t17 * t15968 * t750;
    let t54399 = 3.0_f64 * t54398;
    let t54400 = t2225 * t5166;
    let t54401 = 60.0_f64 * t54400;
    let t54402 = t592 * t15921;
    let t54403 = 24.0_f64 * t54402;
    let t54404 = 0.5848223622634646207e0_f64 * t39478;
    (t54393, t54396, t54399, t54401, t54403, t54404)
}
