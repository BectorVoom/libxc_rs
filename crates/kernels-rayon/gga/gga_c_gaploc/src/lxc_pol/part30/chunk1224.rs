//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1224/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1224(t10673: f64, t10722: f64, t123: f64, t1836: f64, t1841: f64, t1897: f64, t2060: f64, t2508: f64, t2580: f64, t29349: f64, t29354: f64, t32351: f64, t32353: f64, t32355: f64, t32360: f64, t32363: f64, t32364: f64, t32370: f64, t32371: f64, t3433: f64, t3464: f64, t5288: f64, t734: f64, t779: f64) -> f64 {
    let t32386 = t29349 + t32351 - t32353 - t32355 - t32360 + t29354 - t32363 - 0.17090058289204942853e-2_f64 * t1841 * t32364 * t123 * t734 + t32370 + 0.15381052460284448567e-1_f64 * t2508 * t2580 * t32371 + 0.15381052460284448567e-1_f64 * t5288 * t10722 + 0.15381052460284448567e-1_f64 * t2508 * t779 * t10673 + 0.76905262301422242837e-2_f64 * t2508 * t2060 * t3433 - 0.76905262301422242837e-2_f64 * t1897 * t3464 * t1836;
    t32386
}
