//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 871/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk871<F: Float>(t11336: F, t3270: F, t795: F, t1115: F, t481: F, t3493: F, t10656: F, t10923: F, t10932: F, t10944: F, t10947: F, t10956: F, t10964: F, t10969: F, t10973: F, t10982: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t11338 = t3270 * t11336 * t795;
    let t11342 = t3270 * t1115 * t481;
    let t11345 = t3270 * t3493;
    let t11357 = 0.30487649791575028312e-3 * t10656;
    let t11364 = 0.30487649791575028312e-3 * t10923;
    let t11365 = 0.86737941314158990616e-4 * t10932;
    let t11367 = 0.60975299583150056624e-3 * t10944;
    let t11368 = 0.162600798888400151e-2 * t10947;
    let t11372 = 0.162600798888400151e-2 * t10956;
    let t11374 = 0.30487649791575028312e-3 * t10964;
    let t11375 = 0.68400385060046895e-6 * t10969;
    let t11377 = 0.60975299583150056624e-3 * t10973;
    let t11378 = 0.86737941314158990616e-4 * t10982;
    (t11338, t11342, t11345, t11357, t11364, t11365, t11367, t11368, t11372, t11374, t11375, t11377, t11378)
}
