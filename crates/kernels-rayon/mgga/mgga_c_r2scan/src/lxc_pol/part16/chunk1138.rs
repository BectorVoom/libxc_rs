//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1138/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1138(t12557: f64, t3270: f64, t3269: f64, t10619: f64, t12422: f64, t12556: f64, t498: f64, t3275: f64, t3352: f64, t11559: f64, t11629: f64, t11004: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t42439 = t3270 * t12557;
    let t42441 = t3269 * t42439 / 4.0_f64;
    let t42443 = t12422 * t10619 / 4.0_f64;
    let t42444 = t498 * t12556;
    let t42447 = t3275 * t42444 * t3352 / 4.0_f64;
    let t42450 = 5.0_f64 / 8.0_f64 * t3275 * t11629 * t11559;
    let t42452 = 5.0_f64 / 16.0_f64 * t12422 * t11004;
    (t42441, t42443, t42444, t42447, t42450, t42452)
}
