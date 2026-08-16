//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1458/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1458(t12255: f64, t12291: f64, t12297: f64, t12305: f64, t1836: f64, t1885: f64, t1891: f64, t1897: f64, t1901: f64, t2060: f64, t2101: f64, t2508: f64, t29503: f64, t32664: f64, t32669: f64, t32671: f64, t32674: f64, t32676: f64, t3720: f64, t3722: f64, t3732: f64, t39166: f64, t5293: f64, t7129: f64, t779: f64) -> f64 {
    let t39493 = t32664 + t29503 - t32669 + 0.76905262301422242837e-2_f64 * t1897 * t1901 * t39166 + 0.15381052460284448567e-1_f64 * t7129 * t12291 + 0.76905262301422242837e-2_f64 * t2508 * t2060 * t3722 + 0.15381052460284448567e-1_f64 * t2508 * t779 * t12305 - 0.76905262301422242837e-2_f64 * t1897 * t3732 * t1836 - t32671 + t32674 + t32676 + 0.20508069947045931424e-1_f64 * t5293 * t12297 - 0.53833683610995569986e-1_f64 * t2508 * t12255 * t1885 + 0.92286314761706691403e-1_f64 * t2508 * t2101 * t3720 * t1891;
    t39493
}
