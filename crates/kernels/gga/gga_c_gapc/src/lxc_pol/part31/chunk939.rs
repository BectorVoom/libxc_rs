//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 939/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk939<F: Float>(t9326: F, t9331: F, t9334: F, t9337: F, t9339: F, t9341: F, t9344: F, t9346: F, t9349: F, t9351: F, t9354: F, t9357: F, t9360: F, t9364: F) -> F {
    let t10782 = -F::new(0.23590742743871821894e-5) * t9326 - F::new(0.73909120450717768468e-5) * t9331 + F::new(0.15176747947735985782e-6) * t9334 - F::new(0.2698425785107458272e-6) * t9337 - F::new(0.30353495895471971564e-6) * t9339 + F::new(0.53968515702149165441e-6) * t9341 + F::new(0.9275345110817126956e-4) * t9344 - F::new(0.9275345110817126956e-4) * t9346 - F::new(0.17376185052903442709e-3) * t9349 - F::new(0.34752370105806885418e-3) * t9351 + F::new(0.28960308421505737848e-5) * t9354 - F::new(0.17376185052903442709e-3) * t9357 + F::new(0.28960308421505737848e-5) * t9360 + F::new(0.10136107947527008247e-3) * t9364;
    t10782
}
