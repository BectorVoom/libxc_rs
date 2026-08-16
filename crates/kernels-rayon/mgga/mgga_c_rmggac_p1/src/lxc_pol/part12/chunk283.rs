//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 283/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk283(t53: f64, t60: f64, t1375: f64, t1378: f64, t280: f64, t814: f64, t525: f64, t990: f64, t441: f64, t50: f64, t284: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t54 = t53 <= zeta_threshold;
    let t61 = t60 <= zeta_threshold;
    let t1382 = piecewise3(t54, 0.0_f64, -2.0_f64 / 9.0_f64 * t1375 * t280 + 4.0_f64 / 3.0_f64 * t1378 * t814);
    let t1383 = t990 * t525;
    let t1386 = t441 * t50;
    let t1390 = piecewise3(t61, 0.0_f64, -2.0_f64 / 9.0_f64 * t1383 * t284 - 4.0_f64 / 3.0_f64 * t1386 * t814);
    let t1392 = t1382 / 2.0_f64 + t1390 / 2.0_f64;
    (t1383, t1386, t1392)
}
