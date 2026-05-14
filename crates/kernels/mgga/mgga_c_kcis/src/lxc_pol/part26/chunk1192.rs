//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1192/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1192<F: Float>(t8191: F, t97701: F, t28589: F, t5932: F, t22403: F, t7948: F, t28610: F, t5910: F, t102889: F, t102892: F, t102894: F, t102896: F, t102898: F, t102900: F, t102902: F, t3738: F, t7329: F) -> (F, F, F, F, F, F) {
    let t102904 = t97701 * t8191;
    let t102906 = t28589 * t5932;
    let t102908 = t7948 * t22403;
    let t102910 = t28610 * t5910;
    let t102912 = 0.25e0 * t102889 + 0.43166666666666666668e0 * t102892 + 0.53958333333333333333e-1 * t102894 + 0.53958333333333333334e-1 * t102896 + 0.20234375e-1 * t102898 - 1.0 * t102900 + 0.17986111111111111111e-1 * t102902 - 0.1875e0 * t102904 - 0.125e0 * t102906 - 0.625e-1 * t102908 + 0.11111111111111111111e0 * t102910;
    let t102914 = t3738 * t7329;
    (t102904, t102906, t102908, t102910, t102912, t102914)
}
