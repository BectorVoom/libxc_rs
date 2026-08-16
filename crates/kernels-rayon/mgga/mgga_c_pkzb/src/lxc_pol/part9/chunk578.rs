//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 578/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk578(t2393: f64, t326: f64, t2366: f64, t2029: f64, t394: f64, t2369: f64, t758: f64, t405: f64, t466: f64, t178: f64, t404: f64, t53: f64, t931: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2394 = t2393 * t326;
    let t2395 = t2394 * t2366;
    let t2396 = t2029 * t394;
    let t2397 = t2369 * t2396;
    let t2398 = t758 * t2397;
    let t2401 = t466 * t405;
    let t2402 = t178 * t2401;
    let t2404 = 0.47637797908966374413e-4_f64 * t404 * t2402;
    let t2405 = t53 * t931;
    (t2394, t2395, t2396, t2397, t2398, t2401, t2402, t2404, t2405)
}
