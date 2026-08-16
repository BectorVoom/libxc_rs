//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3178/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3178(t12226: f64, t1719: f64, t12470: f64, t1744: f64, t12555: f64, t5180: f64, t12486: f64, t300: f64, t12553: f64, t3521: f64, t1261: f64, t1715: f64, t247: f64, t44701: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t58473 = t1719 * t12226;
    let t58592 = t12470 * t1744;
    let t58647 = t5180 * t12555;
    let t58665 = t300 * t12486;
    let t58672 = t300 * t12553;
    let t58708 = t300 * t3521;
    let t58777 = t1261 * t247 * t44701 * t1715;
    (t58473, t58592, t58647, t58665, t58672, t58708, t58777)
}
