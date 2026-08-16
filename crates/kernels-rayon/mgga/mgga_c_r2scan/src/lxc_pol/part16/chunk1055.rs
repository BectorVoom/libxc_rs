//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1055/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1055(t10648: f64, t10651: f64, t37453: f64, t10972: f64, t37373: f64, t37369: f64, t10977: f64, t10981: f64, t37372: f64, t122: f64, t607: f64, t10928: f64, t3434: f64, t874: f64) -> (f64, f64, f64, f64, f64) {
    let t37455 = t10648 * t37453 * t10651;
    let t37458 = t37373 * t10972;
    let t37459 = 0.45731474687362542471e-3_f64 * t37458;
    let t37460 = t37369 * t10972;
    let t37461 = 0.45731474687362542471e-3_f64 * t37460;
    let t37463 = t37372 * t10977 * t10981;
    let t37464 = 0.65053455985619242968e-4_f64 * t37463;
    let t37465 = t607 * t122;
    let t37468 = t3434 * t10928 * t37465 * t874;
    (t37455, t37459, t37461, t37464, t37468)
}
