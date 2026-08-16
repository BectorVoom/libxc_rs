//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 954/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk954<F: Float>(t11336: F, t3270: F, t795: F, t1115: F, t481: F, t3493: F, t10656: F, t10923: F, t10932: F, t10944: F, t10947: F, t10956: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11338 = t3270 * t11336 * t795;
    let t11342 = t3270 * t1115 * t481;
    let t11345 = t3270 * t3493;
    let t11357 = F::cast_from(0.30487649791575028312e-3_f64) * t10656;
    let t11364 = F::cast_from(0.30487649791575028312e-3_f64) * t10923;
    let t11365 = F::cast_from(0.86737941314158990616e-4_f64) * t10932;
    let t11367 = F::cast_from(0.60975299583150056624e-3_f64) * t10944;
    let t11368 = F::cast_from(0.162600798888400151e-2_f64) * t10947;
    let t11372 = F::cast_from(0.162600798888400151e-2_f64) * t10956;
    (t11338, t11342, t11345, t11357, t11364, t11365, t11367, t11368, t11372)
}
