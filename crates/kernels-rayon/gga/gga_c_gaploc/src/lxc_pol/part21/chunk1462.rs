//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1462/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1462(t12176: f64, t12287: f64, t12291: f64, t12297: f64, t1881: f64, t1897: f64, t2508: f64, t2580: f64, t29631: f64, t32679: f64, t32681: f64, t32683: f64, t32685: f64, t32691: f64, t32695: f64, t32698: f64, t3732: f64, t39095: f64, t5269: f64, t5288: f64, t5293: f64, t702: f64, t7137: f64) -> f64 {
    let t39511 = 0.15381052460284448567e-1_f64 * t5288 * t12297 - 0.15381052460284448567e-1_f64 * t5288 * t12287 - 0.15381052460284448567e-1_f64 * t1897 * t12176 * t702 + 0.20508069947045931424e-1_f64 * t7137 * t12291 - 0.20508069947045931424e-1_f64 * t5293 * t12287 + 0.15381052460284448567e-1_f64 * t5269 * t3732 * t1881 + 0.30762104920568897134e-1_f64 * t2508 * t2580 * t39095 + t29631 - t32679 - t32681 - t32683 + t32685 + t32691 - t32695 - t32698;
    t39511
}
