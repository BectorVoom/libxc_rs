//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 745/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk745<F: Float>(t1759: F, t5219: F, t1871: F, t616: F, t615: F, t1757: F, t1726: F, t611: F, t1727: F, t1745: F, t378: F, t735: F, t1751: F, t1754: F, t159: F, t166: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t5220 = t5219 * t1759;
    let t5222 = t616 * t1871;
    let t5223 = t615 * t5222;
    let t5225 = 0.50808839199999999999e-2 * t1757 * t5223;
    let t5226 = t1726 * t611;
    let t5227 = t616 * t1727;
    let t5228 = t615 * t5227;
    let t5230 = 0.1524265176e-1 * t5226 * t5228;
    let t5231 = t378 * t1745;
    let t5233 = 0.16265371950452609763e-1 * t735 * t5231;
    let t5237 = t1751 * t1754;
    let t5239 = t159 * t166;
    (t5220, t5222, t5223, t5225, t5226, t5227, t5228, t5230, t5231, t5233, t5237, t5239)
}
