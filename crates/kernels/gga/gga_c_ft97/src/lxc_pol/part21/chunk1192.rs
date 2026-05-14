//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1192/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1192<F: Float>(t101710: F, t101712: F, t101719: F, t116679: F, t116683: F, t116688: F, t116692: F, t116695: F, t116699: F, t116703: F, t93504: F, t101768: F, t101772: F, t101779: F, t101782: F, t116708: F, t116710: F, t116712: F, t116716: F, t116720: F, t116724: F, t116728: F, t116729: F) -> (F, F) {
    let t117190 = 8.0 / 81.0 * t101710 - 4.0 / 27.0 * t101712 - t101719 + 2.0 / 81.0 * t93504 + t116679 / 9.0 + 4.0 / 3.0 * t116683 - t116688 / 8.0 - 2.0 * t116692 - 4.0 / 9.0 * t116695 + t116699 / 2.0 + t116703;
    let t117199 = 2.0 / 3.0 * t116708 + t116710 / 27.0 - t101768 - t116712 / 54.0 + t116716 / 18.0 + t116720 / 27.0 + t116724 / 9.0 - t101772 - t101779 - t101782 - t116728 / 3.0 - t116729 / 27.0;
    (t117190, t117199)
}
