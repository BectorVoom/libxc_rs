//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1109/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1109<F: Float>(t1891: F, t47733: F, t639: F, t642: F, t1640: F, t1643: F, t3562: F, t184: F, t209: F, t221: F, t3345: F, t181: F, t199: F) -> (F, F, F, F) {
    let t47737 = F::new(8.0) / F::new(15.0) * t639 * t642 * t1891 * t47733;
    let t47741 = F::new(4.0) / F::new(9.0) * t639 * t1640 * t1643 * t47733;
    let t47742 = t3562 * t3562;
    let t47746 = F::new(4.0) / F::new(5.0) * t47742 * t209 * t184 * t221;
    let t47747 = t3345 * t3345;
    let t47751 = F::new(4.0) / F::new(5.0) * t47747 * t181 * t184 * t199;
    (t47737, t47741, t47746, t47751)
}
