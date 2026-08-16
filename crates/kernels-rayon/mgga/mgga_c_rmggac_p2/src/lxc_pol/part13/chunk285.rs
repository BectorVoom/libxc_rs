//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 285/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk285(t53: f64, t60: f64, t521: f64, t912: f64, t50: f64, t57: f64, t280: f64, t814: f64, t525: f64, t921: f64, t62: f64, t284: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t54 = t53 <= zeta_threshold;
    let t61 = t60 <= zeta_threshold;
    let t1395 = t912 * t521;
    let t1398 = t57 * t50;
    let t1402 = piecewise3(t54, 0.0_f64, 4.0_f64 / 9.0_f64 * t1395 * t280 + 8.0_f64 / 3.0_f64 * t1398 * t814);
    let t1403 = t921 * t525;
    let t1406 = t62 * t50;
    let t1410 = piecewise3(t61, 0.0_f64, 4.0_f64 / 9.0_f64 * t1403 * t284 - 8.0_f64 / 3.0_f64 * t1406 * t814);
    let t1411 = t1402 + t1410;
    (t1395, t1398, t1403, t1406, t1411)
}
