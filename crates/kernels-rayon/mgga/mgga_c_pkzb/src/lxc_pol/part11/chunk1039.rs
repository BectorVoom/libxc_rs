//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1039/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1039(t11532: f64, t942: f64, t11484: f64, t11494: f64, t11497: f64, t1246: f64, t1256: f64, t3904: f64, t3910: f64, t3929: f64, t411: f64, t415: f64) -> (f64, f64) {
    let t11533 = t942 * t11532;
    let t11536 = 0.65854491829355115987e0_f64 * t11484 * t415 - 0.19756347548806534796e1_f64 * t3904 * t1256 + 0.39512695097613069591e1_f64 * t1246 * t3910 - 0.19756347548806534796e1_f64 * t1246 * t3929 - 0.39512695097613069591e1_f64 * t411 * t11494 + 0.39512695097613069591e1_f64 * t411 * t11497 - 0.65854491829355115987e0_f64 * t411 * t11533;
    (t11533, t11536)
}
