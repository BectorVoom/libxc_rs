//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1123/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1123(t12564: f64, t2612: f64, t11032: f64, t3500: f64, t41218: f64, t10848: f64, t3504: f64, t41223: f64, t12660: f64, t7130: f64, t32202: f64, t47874: f64, t47878: f64, t47882: f64, t47886: f64, t47888: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t47890 = 32.0_f64 / 9.0_f64 * t2612 * t12564;
    let t47892 = 16.0_f64 / 15.0_f64 * t11032 * t3500;
    let t47893 = 64.0_f64 / 15.0_f64 * t41218;
    let t47895 = 16.0_f64 / 15.0_f64 * t10848 * t3504;
    let t47896 = 64.0_f64 / 45.0_f64 * t41223;
    let t47898 = 16.0_f64 / 5.0_f64 * t7130 * t12660;
    let t47899 = 16.0_f64 / 81.0_f64 * t32202;
    let t47900 = -t47874 + t47878 + t47882 + t47886 - t47888 - t47890 - t47892 - t47893 - t47895 + t47896 - t47898 - t47899;
    (t47890, t47892, t47893, t47895, t47896, t47898, t47899, t47900)
}
