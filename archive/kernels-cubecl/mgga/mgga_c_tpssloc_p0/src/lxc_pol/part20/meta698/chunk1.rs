//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2665/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2665<F: Float>(t39365: F, t15908: F, t9885: F, t9888: F, t39374: F, t39387: F, t15968: F, t172: F, t763: F, t5154: F, t9713: F, t39360: F, t39364: F, t39373: F, t39384: F, t39393: F, t39397: F, t39400: F, t39408: F, t39411: F) -> (F, F, F, F, F, F, F, F) {
    let t54379 = F::cast_from(0.17090684152272775383e-2_f64) * t39365;
    let t54380 = t15908 * t9885;
    let t54381 = F::cast_from(0.16265371950452609763e-1_f64) * t54380;
    let t54382 = t15908 * t9888;
    let t54383 = F::cast_from(0.48159733137676571078e0_f64) * t54382;
    let t54384 = F::cast_from(0.30762056574649219973e4_f64) * t39374;
    let t54385 = F::cast_from(0.17544670867903938621e1_f64) * t39387;
    let t54387 = t15968 * t172 * t763;
    let t54388 = F::cast_from(0.17544670867903938621e1_f64) * t54387;
    let t54389 = t5154 * t9713;
    let t54390 = F::cast_from(0.5848223622634646207e0_f64) * t54389;
    let t54391 = t39360 + t39364 - t54379 + t54381 + t54383 + t39373 - t54384 - t39384 - t54385 + t39393 - t39397 - t39400 + t39408 + t39411 - t54388 - t54390;
    (t54379, t54381, t54383, t54384, t54385, t54388, t54390, t54391)
}
