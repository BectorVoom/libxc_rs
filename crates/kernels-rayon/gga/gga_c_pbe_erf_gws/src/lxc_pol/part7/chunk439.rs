//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 439/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk439(t1872: f64, t561: f64, t579: f64, t612: f64, t1789: f64, t1797: f64, t1800: f64, t1808: f64, t1814: f64, t1819: f64, t1826: f64, t1831: f64, t1841: f64, t1870: f64, t267: f64) -> (f64, f64, f64) {
    let t1874 = 4.0_f64 / 15.0_f64 * t561 * t1872;
    let t1876 = 4.0_f64 / 15.0_f64 * t579 * t612;
    let t1877 = t1789 + t1797 + t1800 + t1808 + t1814 - t1819 + t1826 - t1831 - t1841 * t267 / 15.0_f64 - t1870 + t1874 - t1876;
    (t1874, t1876, t1877)
}
