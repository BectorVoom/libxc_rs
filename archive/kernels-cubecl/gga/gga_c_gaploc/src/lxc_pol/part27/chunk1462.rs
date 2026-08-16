//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1462/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1462<F: Float>(t12255: F, t12291: F, t12297: F, t12305: F, t1836: F, t1885: F, t1891: F, t1897: F, t1901: F, t2060: F, t2101: F, t2508: F, t29503: F, t32664: F, t32669: F, t32671: F, t32674: F, t32676: F, t3720: F, t3722: F, t3732: F, t39166: F, t5293: F, t7129: F, t779: F) -> F {
    let t39493 = t32664 + t29503 - t32669 + F::cast_from(0.76905262301422242837e-2_f64) * t1897 * t1901 * t39166 + F::cast_from(0.15381052460284448567e-1_f64) * t7129 * t12291 + F::cast_from(0.76905262301422242837e-2_f64) * t2508 * t2060 * t3722 + F::cast_from(0.15381052460284448567e-1_f64) * t2508 * t779 * t12305 - F::cast_from(0.76905262301422242837e-2_f64) * t1897 * t3732 * t1836 - t32671 + t32674 + t32676 + F::cast_from(0.20508069947045931424e-1_f64) * t5293 * t12297 - F::cast_from(0.53833683610995569986e-1_f64) * t2508 * t12255 * t1885 + F::cast_from(0.92286314761706691403e-1_f64) * t2508 * t2101 * t3720 * t1891;
    t39493
}
