//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 996/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk996<F: Float>(t13682: F, t13683: F, t2493: F, t3910: F, t3917: F, t42110: F, t42124: F, t462: F, t53287: F, t81048: F, t81050: F, t81057: F, t88149: F, t88153: F, t88161: F, t88165: F, t88169: F, t88180: F, t88184: F, t88227: F, t88277: F, t88606: F, t9896: F, t9916: F) -> (F,) {
    let t89018 = -12.0 * t462 * t3917 * t88606 + 8.0 * t462 * t3910 * t88184 + 8.0 * t462 * t9896 * t88169 + 8.0 * t462 * t42124 * t88165 + 8.0 / 3.0 * t462 * t3917 * t88149 - 8.0 / 9.0 * t462 * t3910 * t88153 + 8.0 / 3.0 * t13682 * t13683 * t88277 - 8.0 / 3.0 * t462 * t42110 * t88227 - 16.0 / 3.0 * t462 * t9916 * t88161 + 2.0 * t462 * t2493 * t88180 + 8.0 / 3.0 * t81048 - 16.0 / 9.0 * t81050 + 40.0 / 81.0 * t81057 + 112.0 / 81.0 * t53287;
    (t89018,)
}
