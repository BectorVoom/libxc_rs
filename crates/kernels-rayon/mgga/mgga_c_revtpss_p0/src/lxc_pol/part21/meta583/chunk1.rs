//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2295/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2295(t1277: f64, t1774: f64, t3790: f64, t1204: f64, t1811: f64, t1211: f64, t16750: f64, t1209: f64, t5412: f64, t1828: f64, t3568: f64, t1294: f64, t5497: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18084 = t1277 * t1774 * t3790;
    let t18087 = t1204 * t1811;
    let t18090 = t1211 * t16750;
    let t18097 = t1209 * t5412;
    let t18102 = t1828 * t3568;
    let t18103 = t1277 * t18102;
    let t18108 = t5497 * t1294;
    (t18084, t18087, t18090, t18097, t18103, t18108)
}
