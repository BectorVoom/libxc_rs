//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 742/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk742<F: Float>(t12147: F, t3985: F, t1368: F, t3970: F, t3990: F, t1376: F, t1370: F, t3999: F, t1377: F, t3978: F, t1444: F, t451: F, t9: F, t1362: F, t486: F, t3716: F, t503: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t12148 = t12147 * t3985;
    let t12149 = t1368 * t12148;
    let t12151 = t3970 * t3990;
    let t12152 = t1368 * t12151;
    let t12158 = t1376 * t1376;
    let t12159 = 1.0 / t12158;
    let t12185 = t1370 * t3999;
    let t12194 = t3978 * t1377;
    let t12216 = 1.0 / t451 / t1444;
    let t12217 = t9 * t12216;
    let t12229 = t1362 * t1362;
    let t12230 = 1.0 / t12229;
    let t12231 = t486 * t12230;
    let t12234 = 1.0 / t3716 / t503;
    (t12149, t12152, t12159, t12185, t12194, t12217, t12229, t12230, t12231, t12234)
}
