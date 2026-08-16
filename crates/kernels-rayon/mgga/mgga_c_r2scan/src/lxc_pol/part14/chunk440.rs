//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 440/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk440(t188: f64, t1893: f64, t1647: f64, t1891: f64, t644: f64, t652: f64, t621: f64, t650: f64, t1800: f64, t190: f64, t632: f64, t175: f64, t648: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1894 = t188 * t1893;
    let t1897 = 0.51726012919273400301e3_f64 * t1891 * t1894 * t1647;
    let t1898 = t644 * t652;
    let t1901 = 0.32163958997385070134e2_f64 * t650 * t1898 * t621;
    let t1904 = 2.0_f64 * t632 * t190 * t1800;
    let t1906 = 1.0_f64 / t648 / t175;
    (t1894, t1897, t1898, t1901, t1904, t1906)
}
