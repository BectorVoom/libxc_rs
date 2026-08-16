//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 649/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk649(t3718: f64, t797: f64, t1048: f64, t499: f64, t2867: f64, t3275: f64, t3465: f64, t3500: f64, t3504: f64, t3625: f64, t3627: f64, t3630: f64) -> (f64, f64, f64, f64) {
    let t3719 = t3718 * t797;
    let t3721 = t1048 * t499 * t3719;
    let t3722 = t3721 / 4.0_f64;
    let t3724 = t3275 * t3465 * t2867;
    let t3725 = t3724 / 4.0_f64;
    let t3729 = t3500 + t3625 / 4.0_f64 - t3627 / 4.0_f64 + t3630 / 2.0_f64 + t3504;
    (t3719, t3722, t3725, t3729)
}
