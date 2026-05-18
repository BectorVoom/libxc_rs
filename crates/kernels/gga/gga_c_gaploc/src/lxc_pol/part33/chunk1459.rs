//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1459/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1459<F: Float>(t12176: F, t12287: F, t12291: F, t12297: F, t1881: F, t1897: F, t2508: F, t2580: F, t29631: F, t32679: F, t32681: F, t32683: F, t32685: F, t32691: F, t32695: F, t32698: F, t3732: F, t39095: F, t5269: F, t5288: F, t5293: F, t702: F, t7137: F) -> F {
    let t39511 = F::new(0.15381052460284448567e-1) * t5288 * t12297 - F::new(0.15381052460284448567e-1) * t5288 * t12287 - F::new(0.15381052460284448567e-1) * t1897 * t12176 * t702 + F::new(0.20508069947045931424e-1) * t7137 * t12291 - F::new(0.20508069947045931424e-1) * t5293 * t12287 + F::new(0.15381052460284448567e-1) * t5269 * t3732 * t1881 + F::new(0.30762104920568897134e-1) * t2508 * t2580 * t39095 + t29631 - t32679 - t32681 - t32683 + t32685 + t32691 - t32695 - t32698;
    t39511
}
