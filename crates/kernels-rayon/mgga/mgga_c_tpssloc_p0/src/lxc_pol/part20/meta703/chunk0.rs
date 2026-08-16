//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2673/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2673(t39845: f64, t2221: f64, t5166: f64, t2223: f64, t1788: f64, t9216: f64, t9218: f64, t39851: f64, t39855: f64, t39857: f64, t5154: f64, t9494: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t54455 = 180.0_f64 * t39845;
    let t54456 = t2221 * t5166;
    let t54457 = 36.0_f64 * t54456;
    let t54459 = 96.0_f64 * t2223 * t5166;
    let t54460 = t9216 * t1788;
    let t54461 = 240.0_f64 * t54460;
    let t54462 = t9218 * t1788;
    let t54463 = 120.0_f64 * t54462;
    let t54464 = 36.0_f64 * t39851;
    let t54465 = 480.0_f64 * t39855;
    let t54466 = 96.0_f64 * t39857;
    let t54467 = t5154 * t9494;
    (t54455, t54457, t54459, t54461, t54463, t54464, t54465, t54466, t54467)
}
