//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 927/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk927<F: Float>(t12137: F, t1368: F, t24: F, t3977: F, t3980: F, t25: F, t4002: F, t493: F, t1377: F, t3970: F, t3985: F, t3990: F, t1376: F, t1370: F, t3999: F, t3978: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t12138 = t1368 * t12137;
    let t12140 = t24 * t3977;
    let t12141 = t12140 * t3980;
    let t12142 = t1368 * t12141;
    let t12144 = t25 * t4002;
    let t12145 = t493 * t12144;
    let t12147 = t3970 * t1377;
    let t12148 = t12147 * t3985;
    let t12149 = t1368 * t12148;
    let t12151 = t3970 * t3990;
    let t12152 = t1368 * t12151;
    let t12158 = t1376 * t1376;
    let t12159 = 1.0 / t12158;
    let t12185 = t1370 * t3999;
    let t12194 = t3978 * t1377;
    (t12138, t12140, t12142, t12145, t12147, t12149, t12152, t12159, t12185, t12194)
}
