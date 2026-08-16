//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 888/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk888(t587: f64, t7695: f64, t1660: f64, t331: f64, t197: f64, t7346: f64, t1802: f64, t1885: f64, t1017: f64, t562: f64, t610: f64, t1820: f64) -> (f64, f64, f64) {
    let t7697 = 32.0_f64 / 45.0_f64 * t587 * t7695;
    let t7698 = t331 * t1660;
    let t7699 = t7698 * t197;
    let t7700 = t7699 * t7346;
    let t7702 = 16.0_f64 / 27.0_f64 * t587 * t7700;
    let t7703 = t1885 * t1802;
    let t7704 = t1017 * t562;
    let t7705 = t7704 * t610;
    let t7706 = t7703 * t7705;
    let t7708 = 16.0_f64 / 15.0_f64 * t1820 * t7706;
    (t7697, t7702, t7708)
}
