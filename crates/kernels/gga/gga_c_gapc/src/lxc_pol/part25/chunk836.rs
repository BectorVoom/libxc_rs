//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 836/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk836<F: Float>(t9326: F, t9331: F, t9334: F, t9337: F, t9339: F, t9341: F, t9344: F, t9346: F, t9349: F, t9351: F, t9354: F, t9357: F, t9360: F, t9364: F, t10560: F, t10574: F, t10589: F, t10603: F, t10619: F, t10633: F, t10648: F, t10662: F, t10679: F, t10693: F, t10708: F, t10722: F, t10738: F, t10752: F, t10767: F) -> (F,) {
    let t10782 = -0.23590742743871821894e-5 * t9326 - 0.73909120450717768468e-5 * t9331 + 0.15176747947735985782e-6 * t9334 - 0.2698425785107458272e-6 * t9337 - 0.30353495895471971564e-6 * t9339 + 0.53968515702149165441e-6 * t9341 + 0.9275345110817126956e-4 * t9344 - 0.9275345110817126956e-4 * t9346 - 0.17376185052903442709e-3 * t9349 - 0.34752370105806885418e-3 * t9351 + 0.28960308421505737848e-5 * t9354 - 0.17376185052903442709e-3 * t9357 + 0.28960308421505737848e-5 * t9360 + 0.10136107947527008247e-3 * t9364;
    let t10786 = t10560 + t10574 + t10589 + t10603 + t10619 + t10633 + t10648 + t10662 + t10679 + t10693 + t10708 + t10722 + t10738 + t10752 + t10767 + t10782;
    (t10786,)
}
