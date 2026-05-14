//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 994/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk994<F: Float>(t10420: F, t10481: F, t2868: F, t302: F, t36748: F, t36754: F, t36756: F, t38060: F, t4041: F, t47690: F, t47694: F, t47698: F, t47702: F, t47706: F, t47710: F, t47714: F, t47719: F, t5055: F, t5928: F, t72: F, t9318: F, t9321: F, t9340: F) -> (F,) {
    let t49649 = 0.35922725105591425692e0 * t5055 * t9318 + 0.23948483403727617128e0 * t2868 * t9321 + t72 * t302 * t10481 - 0.30487649791575028312e-3 * t36748 - t38060 - 0.30487649791575028312e-3 * t36754 + 0.60975299583150056624e-3 * t36756 + 0.1064114997332445985e-4 * t47690 + 0.43368970657079495308e-4 * t47694 - 0.30487649791575028312e-3 * t47698 - 0.60975299583150056624e-3 * t47702 + 0.86737941314158990616e-4 * t47706 - 0.30487649791575028312e-3 * t47710 + 0.43368970657079495308e-4 * t47714 + 0.59871208509319042821e-1 * t4041 * t10420 - 0.47896966807455234256e0 * t47719 + 0.79828278012425390428e-1 * t5928 * t9340;
    (t49649,)
}
