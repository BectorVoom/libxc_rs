//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 954/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk954(t11336: f64, t3270: f64, t795: f64, t1115: f64, t481: f64, t3493: f64, t10656: f64, t10923: f64, t10932: f64, t10944: f64, t10947: f64, t10956: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11338 = t3270 * t11336 * t795;
    let t11342 = t3270 * t1115 * t481;
    let t11345 = t3270 * t3493;
    let t11357 = 0.30487649791575028312e-3_f64 * t10656;
    let t11364 = 0.30487649791575028312e-3_f64 * t10923;
    let t11365 = 0.86737941314158990616e-4_f64 * t10932;
    let t11367 = 0.60975299583150056624e-3_f64 * t10944;
    let t11368 = 0.162600798888400151e-2_f64 * t10947;
    let t11372 = 0.162600798888400151e-2_f64 * t10956;
    (t11338, t11342, t11345, t11357, t11364, t11365, t11367, t11368, t11372)
}
