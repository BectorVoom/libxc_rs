//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1004/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1004<F: Float>(t3695: F, t590: F, t1339: F, t3689: F, t1589: F, t12001: F, t531: F, t10446: F, t10450: F, t10457: F, t10459: F, t10465: F, t10468: F, t10472: F, t10476: F, t1441: F, t1537: F, t1599: F, t3705: F, t557: F, t9539: F, t9545: F) -> (F, F, F, F, F) {
    let t12089 = t3695 * t590;
    let t12092 = t1339 * t3689;
    let t12093 = t12092 * t590;
    let t12098 = t1589 * t3695;
    let t12103 = t531 * t12001;
    let t12106 = -t10446 - t10450 + t10457 + t10459 + t10465 + F::cast_from(0.51123901271894332902e0_f64) * t1441 * t12089 - F::cast_from(0.51123901271894332902e0_f64) * t1537 * t12093 - t10468 - t10472 + t10476 - F::cast_from(0.38342925953920749677e0_f64) * t9539 + F::cast_from(0.38342925953920749677e0_f64) * t9545 - F::cast_from(0.23833659967900284446e0_f64) * t557 * t12098 - F::cast_from(0.35750489951850426669e0_f64) * t1599 * t3705 - F::cast_from(0.35750489951850426669e0_f64) * t557 * t12103;
    (t12089, t12093, t12098, t12103, t12106)
}
