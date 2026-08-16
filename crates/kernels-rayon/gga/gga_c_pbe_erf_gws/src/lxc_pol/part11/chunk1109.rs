//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1109/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1109(t1891: f64, t47733: f64, t639: f64, t642: f64, t1640: f64, t1643: f64, t3562: f64, t184: f64, t209: f64, t221: f64, t3345: f64, t181: f64, t199: f64) -> (f64, f64, f64, f64) {
    let t47737 = 8.0_f64 / 15.0_f64 * t639 * t642 * t1891 * t47733;
    let t47741 = 4.0_f64 / 9.0_f64 * t639 * t1640 * t1643 * t47733;
    let t47742 = t3562 * t3562;
    let t47746 = 4.0_f64 / 5.0_f64 * t47742 * t209 * t184 * t221;
    let t47747 = t3345 * t3345;
    let t47751 = 4.0_f64 / 5.0_f64 * t47747 * t181 * t184 * t199;
    (t47737, t47741, t47746, t47751)
}
