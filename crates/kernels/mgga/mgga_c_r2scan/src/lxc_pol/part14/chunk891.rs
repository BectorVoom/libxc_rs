//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 891/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk891<F: Float>(t10923: F, t10932: F, t10944: F, t10947: F, t10678: F, t10685: F, t10692: F, t10695: F, t10937: F, t11205: F, t11328: F, t10956: F, t10964: F, t10969: F, t10952: F, t10960: F, t11335: F, t11340: F, t11344: F, t11347: F, t11350: F, t11352: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11364 = 0.30487649791575028312e-3 * t10923;
    let t11365 = 0.86737941314158990616e-4 * t10932;
    let t11367 = 0.60975299583150056624e-3 * t10944;
    let t11368 = 0.162600798888400151e-2 * t10947;
    let t11369 = t11205 - 0.20496175532535769482e-3 * t10678 + 0.1440846329149835838e-2 * t10685 + t11328 - 0.72042316457491791901e-3 * t10692 - 0.1440846329149835838e-2 * t10695 + t11364 - t11365 + 0.3842256877732895568e-2 * t10937 + t11367 + t11368;
    let t11372 = 0.162600798888400151e-2 * t10956;
    let t11374 = 0.30487649791575028312e-3 * t10964;
    let t11375 = 0.68400385060046895e-6 * t10969;
    let t11376 = 0.86737941314158990616e-4 * t10952 + t11372 - 0.60975299583150056624e-3 * t10960 - t11374 - t11335 + t11340 - t11344 - t11347 - t11350 - t11352 + t11375;
    (t11364, t11365, t11367, t11368, t11369, t11372, t11374, t11375, t11376)
}
