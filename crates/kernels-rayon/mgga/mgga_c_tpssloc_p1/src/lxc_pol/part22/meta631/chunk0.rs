//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2166/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2166(t5154: f64, t9722: f64, t39659: f64, t2221: f64, t5166: f64, t2223: f64, t1788: f64, t9216: f64, t9218: f64, t39855: f64, t39857: f64, t9494: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t54451 = t5154 * t9722;
    let t54453 = 96.0_f64 * t39659;
    let t54456 = t2221 * t5166;
    let t54457 = 36.0_f64 * t54456;
    let t54459 = 96.0_f64 * t2223 * t5166;
    let t54460 = t9216 * t1788;
    let t54461 = 240.0_f64 * t54460;
    let t54462 = t9218 * t1788;
    let t54465 = 480.0_f64 * t39855;
    let t54466 = 96.0_f64 * t39857;
    let t54467 = t5154 * t9494;
    (t54451, t54453, t54457, t54459, t54461, t54462, t54465, t54466, t54467)
}
