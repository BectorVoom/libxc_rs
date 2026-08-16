//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1028/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1028(t2298: f64, t358: f64, t364: f64, t2316: f64, t818: f64, t8353: f64, t1275: f64, t2376: f64, t1004: f64, t6660: f64, t2333: f64, t8299: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t23099 = t2298 * t2298;
    let t23102 = t358 / t364 / t23099;
    let t23193 = t2316 * t2316;
    let t23194 = 1.0_f64 / t23193;
    let t23353 = t8353 * t818;
    let t23495 = t2376 * t1275;
    let t23498 = t1004 * t6660;
    let t23754 = t8299 * t2333;
    (t23102, t23194, t23353, t23495, t23498, t23754)
}
