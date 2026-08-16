//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 801/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk801(t301: f64, t6308: f64, t960: f64, t1879: f64, t372: f64, t1150: f64, t1173: f64, t335: f64, t367: f64, t3671: f64, t3673: f64, t3677: f64, t3679: f64, t3694: f64, t3699: f64, t3702: f64, t5169: f64, t5175: f64, t6271: f64, t6279: f64, t6283: f64, t6286: f64, t6290: f64, t6294: f64, t6297: f64, t6301: f64, t6305: f64) -> (f64, f64, f64, f64, f64) {
    let t6309 = t6308 * t301;
    let t6310 = t960 * t6309;
    let t6313 = t1879 * t372;
    let t6314 = t960 * t6313;
    let t6317 = 0.34299214494455789578e-2_f64 * t1173 * t6271 - t5169 - t5175 - 0.22675591804667994221e-1_f64 * t3671 + 0.11337795902333997111e-1_f64 * t3673 - 0.11337795902333997111e-1_f64 * t3677 + 0.80031500487063509016e-2_f64 * t3679 - t3694 - t3699 - t3702 + t367 * t6279 / 48.0_f64 + t1150 * t6283 / 16.0_f64 - 7.0_f64 / 48.0_f64 * t6286 + t335 * t6290 / 48.0_f64 + t367 * t6294 / 48.0_f64 + t335 * t6297 / 24.0_f64 + t335 * t6301 / 24.0_f64 + t335 * t6305 / 24.0_f64 - t335 * t6310 / 24.0_f64 - t367 * t6314 / 16.0_f64;
    (t6309, t6310, t6313, t6314, t6317)
}
