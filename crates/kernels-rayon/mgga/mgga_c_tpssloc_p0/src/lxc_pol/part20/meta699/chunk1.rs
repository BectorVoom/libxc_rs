//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2667/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2667(t15977: f64, t592: f64, t17: f64, t2516: f64, t5151: f64, t1787: f64, t9861: f64, t15971: f64, t39491: f64, t39463: f64, t39468: f64, t39472: f64, t39476: f64, t39483: f64, t39490: f64, t54393: f64, t54396: f64, t54399: f64, t54401: f64, t54403: f64, t54404: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t54405 = t592 * t15977;
    let t54406 = 12.0_f64 * t54405;
    let t54408 = t17 * t5151 * t2516;
    let t54409 = 3.0_f64 * t54408;
    let t54411 = t17 * t1787 * t9861;
    let t54412 = t592 * t15971;
    let t54413 = 12.0_f64 * t54412;
    let t54414 = 0.35089341735807877242e1_f64 * t39491;
    let t54415 = t39463 - t39468 + t54393 - t54396 + t54399 + t54401 - t39472 - t39476 - t54403 - t54404 - t54406 + t54409 + t54411 + t39483 - t54413 - t39490 + t54414;
    (t54406, t54409, t54411, t54413, t54414, t54415)
}
