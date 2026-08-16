//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 775/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk775(t805: f64, t9541: f64, t2563: f64, t2610: f64, t119: f64, t210: f64, t9516: f64, t10009: f64, t10012: f64, t10014: f64, t10017: f64, t10026: f64, t10029: f64, t10030: f64, t10033: f64, t249: f64, t2643: f64, t787: f64, t9559: f64) -> (f64, f64) {
    let t10036 = t9541 * t805;
    let t10038 = t2563 * t2610;
    let t10041 = t210 * t119 * t9516;
    let t10044 = t2643 * t10009 / 256.0_f64 - 7.0_f64 / 1536.0_f64 * t10012 + 119.0_f64 / 4608.0_f64 * t10014 + t10017 * t249 / 3072.0_f64 - t10026 - t10029 - 7.0_f64 / 16.0_f64 * t10030 - t9559 * t10033 / 4.0_f64 - 35.0_f64 / 72.0_f64 * t10036 + 7.0_f64 / 48.0_f64 * t10038 - t787 * t10041 / 48.0_f64;
    (t10041, t10044)
}
