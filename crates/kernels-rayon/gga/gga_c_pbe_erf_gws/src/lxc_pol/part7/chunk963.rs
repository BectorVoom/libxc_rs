//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 963/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk963(t17817: f64, t197: f64, t4991: f64, t1820: f64, t1823: f64, t1672: f64, t1871: f64, t561: f64, t5280: f64, t579: f64, t1733: f64, t184: f64, t209: f64, t221: f64) -> (f64, f64, f64, f64, f64) {
    let t17818 = 32.0_f64 / 27.0_f64 * t17817;
    let t17819 = t4991 * t197;
    let t17821 = t1820 * t17819 * t1823;
    let t17822 = 64.0_f64 / 135.0_f64 * t17821;
    let t17824 = t561 * t1672 * t1871;
    let t17825 = 16.0_f64 / 45.0_f64 * t17824;
    let t17826 = t579 * t5280;
    let t17827 = 16.0_f64 / 15.0_f64 * t17826;
    let t17828 = t1733 * t1733;
    let t17832 = 4.0_f64 / 5.0_f64 * t17828 * t209 * t184 * t221;
    (t17818, t17822, t17825, t17827, t17832)
}
