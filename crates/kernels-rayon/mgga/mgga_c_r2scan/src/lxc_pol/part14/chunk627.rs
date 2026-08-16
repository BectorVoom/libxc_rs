//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 627/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk627(t322: f64, t1048: f64, t3493: f64, t499: f64, t3275: f64, t3352: f64, t3465: f64, t3356: f64, t3367: f64, t3359: f64, t3361: f64, t3364: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t324 = 0.0_f64 < t322;
    let t3495 = t1048 * t499 * t3493;
    let t3496 = t3495 / 4.0_f64;
    let t3498 = t3275 * t3465 * t3352;
    let t3499 = t3498 / 4.0_f64;
    let t3500 = 2.0_f64 / 3.0_f64 * t3356;
    let t3504 = 2.0_f64 / 3.0_f64 * t3367;
    let t3505 = t3500 + t3359 / 4.0_f64 - t3361 / 4.0_f64 + t3364 / 2.0_f64 + t3504;
    let t3506 = piecewise3(t324, 0.0_f64, t3505);
    (t3496, t3499, t3500, t3504, t3505, t3506)
}
