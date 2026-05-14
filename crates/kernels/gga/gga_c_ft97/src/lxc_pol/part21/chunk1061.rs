//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1061/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1061<F: Float>(t101708: F, t6513: F, t8232: F, t12001: F, t25961: F, t1882: F, t25943: F, t1900: F, t6: F, t8345: F, t91: F, t358: F, t965: F, t26012: F, t376: F, t5665: F) -> (F, F, F, F, F, F, F, F) {
    let t101709 = 2.0 / 27.0 * t101708;
    let t101710 = t8232 * t6513;
    let t101712 = t12001 * t25961;
    let t101718 = t1882 * t25943;
    let t101719 = 2.0 / 27.0 * t101718;
    let t101733 = t91 * t8345 * t6 * t1900;
    let t101734 = t965 * t358;
    let t101767 = t5665 * t376 * t26012;
    (t101709, t101710, t101712, t101718, t101719, t101733, t101734, t101767)
}
