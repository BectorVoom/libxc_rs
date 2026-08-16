//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2667/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2667<F: Float>(t15977: F, t592: F, t17: F, t2516: F, t5151: F, t1787: F, t9861: F, t15971: F, t39491: F, t39463: F, t39468: F, t39472: F, t39476: F, t39483: F, t39490: F, t54393: F, t54396: F, t54399: F, t54401: F, t54403: F, t54404: F) -> (F, F, F, F, F, F) {
    let t54405 = t592 * t15977;
    let t54406 = F::cast_from(12.0_f64) * t54405;
    let t54408 = t17 * t5151 * t2516;
    let t54409 = F::cast_from(3.0_f64) * t54408;
    let t54411 = t17 * t1787 * t9861;
    let t54412 = t592 * t15971;
    let t54413 = F::cast_from(12.0_f64) * t54412;
    let t54414 = F::cast_from(0.35089341735807877242e1_f64) * t39491;
    let t54415 = t39463 - t39468 + t54393 - t54396 + t54399 + t54401 - t39472 - t39476 - t54403 - t54404 - t54406 + t54409 + t54411 + t39483 - t54413 - t39490 + t54414;
    (t54406, t54409, t54411, t54413, t54414, t54415)
}
