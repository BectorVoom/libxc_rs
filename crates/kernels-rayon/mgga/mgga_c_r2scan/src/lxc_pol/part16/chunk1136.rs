//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1136/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1136(t11523: f64, t11540: f64, t2333: f64, t3060: f64, t795: f64, t10997: f64, t3275: f64, t3229: f64, t3276: f64, t792: f64, t8601: f64, t12414: f64) -> (f64, f64, f64, f64, f64) {
    let t42417 = t11523 * t11540 / 2.0_f64;
    let t42418 = t2333 * t3060;
    let t42419 = t42418 * t795;
    let t42422 = 45.0_f64 / 64.0_f64 * t3275 * t10997 * t42419;
    let t42423 = t2333 * t3229;
    let t42424 = t42423 * t795;
    let t42427 = 5.0_f64 / 16.0_f64 * t3275 * t3276 * t42424;
    let t42428 = t8601 * t792;
    let t42431 = 5.0_f64 / 16.0_f64 * t3275 * t3276 * t42428;
    let t42432 = t12414 * t792;
    (t42417, t42422, t42427, t42431, t42432)
}
