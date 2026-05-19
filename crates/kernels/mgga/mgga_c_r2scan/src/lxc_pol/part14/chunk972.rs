//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 972/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk972<F: Float>(t10956: F, t10964: F, t10969: F, t10952: F, t10960: F, t11335: F, t11340: F, t11344: F, t11347: F, t11350: F, t11352: F, t10973: F) -> (F, F, F, F, F) {
    let t11372 = F::cast_from(0.162600798888400151e-2_f64) * t10956;
    let t11374 = F::cast_from(0.30487649791575028312e-3_f64) * t10964;
    let t11375 = F::cast_from(0.68400385060046895e-6_f64) * t10969;
    let t11376 = F::cast_from(0.86737941314158990616e-4_f64) * t10952 + t11372 - F::cast_from(0.60975299583150056624e-3_f64) * t10960 - t11374 - t11335 + t11340 - t11344 - t11347 - t11350 - t11352 + t11375;
    let t11377 = F::cast_from(0.60975299583150056624e-3_f64) * t10973;
    (t11372, t11374, t11375, t11376, t11377)
}
