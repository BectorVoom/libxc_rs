//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 979/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk979<F: Float>(t1891: F, t47733: F, t639: F, t642: F, t1640: F, t1643: F, t3562: F, t184: F, t209: F, t221: F, t3345: F, t181: F, t199: F, t40824: F, t23817: F, t47727: F, t47728: F, t47729: F, t47730: F, t47731: F, t47732: F) -> (F, F, F, F, F, F, F) {
    let t47737 = 8.0 / 15.0 * t639 * t642 * t1891 * t47733;
    let t47741 = 4.0 / 9.0 * t639 * t1640 * t1643 * t47733;
    let t47742 = t3562 * t3562;
    let t47746 = 4.0 / 5.0 * t47742 * t209 * t184 * t221;
    let t47747 = t3345 * t3345;
    let t47751 = 4.0 / 5.0 * t47747 * t181 * t184 * t199;
    let t47752 = 32.0 / 27.0 * t40824;
    let t47753 = 128.0 / 1215.0 * t23817;
    let t47754 = -t47727 + t47728 + t47729 + t47730 - t47731 - t47732 - t47737 + t47741 + t47746 + t47751 + t47752 + t47753;
    (t47737, t47741, t47746, t47751, t47752, t47753, t47754)
}
