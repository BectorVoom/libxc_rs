//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 391/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk391(t1866: f64, t960: f64, t1514: f64, t1516: f64, t1542: f64, t1565: f64, t1817: f64, t1841: f64, t1846: f64, t1851: f64, t1856: f64, t1861: f64, t335: f64, t397: f64, t418: f64, t942: f64) -> (f64, f64) {
    let t1867 = t960 * t1866;
    let t1872 = 0.42874018118069736972e-3_f64 * t942 * t1817 - 0.21437009059034868486e-3_f64 * t397 * t1841 - 0.21437009059034868486e-3_f64 * t397 * t1846 + 0.34299214494455789578e-2_f64 * t418 * t1851 - 0.17149607247227894789e-2_f64 * t418 * t1856 - 0.34299214494455789578e-2_f64 * t418 * t1861 + 7.0_f64 / 144.0_f64 * t1514 + 7.0_f64 / 72.0_f64 * t1516 + t335 * t1867 / 24.0_f64 + 0.42874018118069736972e-3_f64 * t1542 - 0.17149607247227894789e-2_f64 * t1565;
    (t1867, t1872)
}
