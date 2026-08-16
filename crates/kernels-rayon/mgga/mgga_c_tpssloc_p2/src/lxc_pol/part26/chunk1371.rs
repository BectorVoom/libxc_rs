//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1371/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1371(t10401: f64, t24739: f64, t3610: f64, t3624: f64, t24740: f64, t3604: f64, t11838: f64, t7310: f64, t11841: f64, t11496: f64, t11670: f64, t11674: f64, t11680: f64, t11684: f64, t11688: f64, t11694: f64, t11845: f64, t2134: f64, t24741: f64, t3580: f64, t460: f64, t7320: f64) -> f64 {
    let t86323 = t24739 * t10401;
    let t86324 = t3610 * t86323;
    let t86327 = t3624 * t86323;
    let t86330 = t3604 * t24740;
    let t86341 = t7310 * t11838;
    let t86343 = t7310 * t11841;
    let t86347 = 5.0_f64 / 2304.0_f64 * t24741 * t11670 - t24741 * t11684 / 768.0_f64 - t86324 * t11680 / 384.0_f64 + t86327 * t11694 / 768.0_f64 - t86330 * t3580 / 384.0_f64 - t24741 * t11674 / 768.0_f64 - t24741 * t11688 / 384.0_f64 - 0.10093189023535097714e-3_f64 * t2134 * t11496 * t460 * t7320 - t86341 / 288.0_f64 - t86343 / 144.0_f64 - t7310 * t11845 / 288.0_f64;
    t86347
}
