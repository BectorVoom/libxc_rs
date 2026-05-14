//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 650/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk650<F: Float>(t1726: F, t611: F, t1727: F, t616: F, t615: F, t1745: F, t378: F, t735: F, t1751: F, t1754: F, t159: F, t166: F, t15: F, t3: F, t42: F, t148: F) -> (F, F, F, F, F) {
    let t5226 = t1726 * t611;
    let t5227 = t616 * t1727;
    let t5228 = t615 * t5227;
    let t5230 = 0.1524265176e-1 * t5226 * t5228;
    let t5231 = t378 * t1745;
    let t5233 = 0.16265371950452609763e-1 * t735 * t5231;
    let t5237 = t1751 * t1754;
    let t5239 = t159 * t166;
    let t5243 = 1.0 / t15 / t3 / t42 / 48.0;
    let t5244 = t148 * t5243;
    (t5230, t5233, t5237, t5239, t5244)
}
