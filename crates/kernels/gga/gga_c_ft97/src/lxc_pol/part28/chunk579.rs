//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 579/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk579<F: Float>(t25915: F, t28: F, t89: F, t3204: F, t5691: F, t22958: F, t5674: F, t25873: F, t25876: F, t25881: F, t25886: F, t25891: F, t25897: F, t25902: F, t25906: F, t25910: F, t25913: F) -> (F, F, F, F, F) {
    let t25916 = t28 * t25915;
    let t25917 = t89 * t25916;
    let t25919 = t5691 * t3204;
    let t25920 = t22958 * t25919;
    let t25921 = t5674 * t25920;
    let t25923 = -t25873 + t25876 / 18.0 + t25881 / 9.0 - t25886 / 6.0 - t25891 / 6.0 - t25897 / 8.0 + t25902 / 18.0 + 2.0 / 3.0 * t25906 + 2.0 / 3.0 * t25910 - 2.0 / 9.0 * t25913 + 2.0 / 3.0 * t25917 - t25921 / 9.0;
    (t25916, t25917, t25919, t25921, t25923)
}
