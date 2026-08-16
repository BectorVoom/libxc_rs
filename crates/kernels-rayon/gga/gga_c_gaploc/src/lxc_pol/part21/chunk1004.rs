//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1004/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1004(t3695: f64, t590: f64, t1339: f64, t3689: f64, t1589: f64, t12001: f64, t531: f64, t10446: f64, t10450: f64, t10457: f64, t10459: f64, t10465: f64, t10468: f64, t10472: f64, t10476: f64, t1441: f64, t1537: f64, t1599: f64, t3705: f64, t557: f64, t9539: f64, t9545: f64) -> (f64, f64, f64, f64, f64) {
    let t12089 = t3695 * t590;
    let t12092 = t1339 * t3689;
    let t12093 = t12092 * t590;
    let t12098 = t1589 * t3695;
    let t12103 = t531 * t12001;
    let t12106 = -t10446 - t10450 + t10457 + t10459 + t10465 + 0.51123901271894332902e0_f64 * t1441 * t12089 - 0.51123901271894332902e0_f64 * t1537 * t12093 - t10468 - t10472 + t10476 - 0.38342925953920749677e0_f64 * t9539 + 0.38342925953920749677e0_f64 * t9545 - 0.23833659967900284446e0_f64 * t557 * t12098 - 0.35750489951850426669e0_f64 * t1599 * t3705 - 0.35750489951850426669e0_f64 * t557 * t12103;
    (t12089, t12093, t12098, t12103, t12106)
}
