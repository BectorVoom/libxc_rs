//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1227/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1227<F: Float>(t10673: F, t10722: F, t123: F, t1836: F, t1841: F, t1897: F, t2060: F, t2508: F, t2580: F, t29349: F, t29354: F, t32351: F, t32353: F, t32355: F, t32360: F, t32363: F, t32364: F, t32370: F, t32371: F, t3433: F, t3464: F, t5288: F, t734: F, t779: F) -> F {
    let t32386 = t29349 + t32351 - t32353 - t32355 - t32360 + t29354 - t32363 - F::cast_from(0.17090058289204942853e-2_f64) * t1841 * t32364 * t123 * t734 + t32370 + F::cast_from(0.15381052460284448567e-1_f64) * t2508 * t2580 * t32371 + F::cast_from(0.15381052460284448567e-1_f64) * t5288 * t10722 + F::cast_from(0.15381052460284448567e-1_f64) * t2508 * t779 * t10673 + F::cast_from(0.76905262301422242837e-2_f64) * t2508 * t2060 * t3433 - F::cast_from(0.76905262301422242837e-2_f64) * t1897 * t3464 * t1836;
    t32386
}
