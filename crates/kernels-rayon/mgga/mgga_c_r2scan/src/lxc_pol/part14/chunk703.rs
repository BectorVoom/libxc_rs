//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 703/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk703(t4738: f64, t640: f64, t4741: f64, t5246: f64, t5416: f64, t5418: f64, t5422: f64, t5424: f64, t219: f64, t225: f64, t234: f64, t61: f64, t704: f64) -> (f64, f64, f64, f64) {
    let t5426 = t640 * t4738;
    let t5429 = 0.17261666666666666666e2_f64 * t5246 - 0.69046666666666666665e1_f64 * t5416 + 0.10740592592592592593e2_f64 * t5418 - 0.44012999999999999999e0_f64 * t5422 + 0.29342e0_f64 * t5424 - 0.34232333333333333333e0_f64 * t5426 - 0.25755333333333333333e0_f64 * t4741;
    let t5431 = t219 * t5429 * t225;
    let t5433 = 0.5848223622634646207e0_f64 * t234 * t5431;
    let t5434 = t61 * t704;
    (t5426, t5429, t5433, t5434)
}
