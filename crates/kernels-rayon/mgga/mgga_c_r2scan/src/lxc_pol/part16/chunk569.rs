//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 569/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk569(t1856: f64, t1858: f64, t1874: f64, t1885: f64, t1888: f64, t1897: f64, t1901: f64, t1904: f64, t1910: f64, t1913: f64, t1916: f64, t2037: f64, t2789: f64, t2795: f64, t2800: f64) -> f64 {
    let t3156 = -t1856 + t1858 - 0.10843581300301739842e-1_f64 * t2789 - 2.0_f64 * t2795 - t1874 - t1885 - t1888 - t1897 - t1901 + t1904 + t1910 + t1913 - t1916 + 0.16936279733333333333e-2_f64 * t2800 - t2037;
    t3156
}
