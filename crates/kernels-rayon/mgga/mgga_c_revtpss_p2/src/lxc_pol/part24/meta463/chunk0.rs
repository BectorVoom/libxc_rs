//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1435/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1435(t12485: f64, t1749: f64, t12428: f64, t1737: f64, t12247: f64, t1719: f64, t12226: f64, t1261: f64, t1715: f64, t247: f64, t44701: f64, t1247: f64, t1796: f64, t42994: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t58262 = t1749 * t12485;
    let t58304 = t1737 * t12428;
    let t58342 = t1719 * t12247;
    let t58473 = t1719 * t12226;
    let t58777 = t1261 * t247 * t44701 * t1715;
    let t58824 = t1247 * t42994 * t1796;
    (t58262, t58304, t58342, t58473, t58777, t58824)
}
