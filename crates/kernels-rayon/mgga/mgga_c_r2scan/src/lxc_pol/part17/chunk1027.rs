//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1027/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1027(t322: f64, t12828: f64, t10533: f64, t11305: f64, t11319: f64, t12348: f64, t12355: f64, t12683: f64, t12849: f64, t12851: f64, t12854: f64, t12856: f64, t12883: f64, t12908: f64, t330: f64, t352: f64, t3549: f64, t3556: f64, t3675: f64, t855: f64) -> (f64, f64) {
    let t323 = t322 <= 0.0_f64;
    let t331 = t322 <= 0.25e1_f64;
    let t332 = 0.25e1_f64 < t322;
    let t12918 = piecewise3(t332, t12828, 0.0_f64);
    let t12929 = piecewise5(t323, t12849 * t330 + 2.0_f64 * t12851 * t330 + t12854 * t330 + t12856 * t330, t331, t12883 + t12908, -0.63e1_f64 * t3556 * t12683 - 0.42e1_f64 * t12348 * t3675 - 0.945e1_f64 * t11305 * t12683 - 0.21e1_f64 * t3549 * t10533 - 0.105e1_f64 * t855 * t12918 * t352 - 0.315e1_f64 * t12355 * t3675 - 0.1575e1_f64 * t3556 * t10533 - 0.23625e1_f64 * t11319 * t12683);
    (t12918, t12929)
}
