//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2582/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2582(t12553: f64, t300: f64, t3521: f64, t1261: f64, t1715: f64, t247: f64, t44701: f64, t1247: f64, t1796: f64, t42994: f64, t3718: f64, t44546: f64, t5347: f64) -> (f64, f64, f64, f64, f64) {
    let t58672 = t300 * t12553;
    let t58708 = t300 * t3521;
    let t58777 = t1261 * t247 * t44701 * t1715;
    let t58824 = t1247 * t42994 * t1796;
    let t58850 = t3718 * t44546 * t5347;
    (t58672, t58708, t58777, t58824, t58850)
}
