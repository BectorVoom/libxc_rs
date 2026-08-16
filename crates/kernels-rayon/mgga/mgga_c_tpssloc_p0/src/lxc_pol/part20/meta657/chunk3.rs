//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2431/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2431(t10883: f64, t13969: f64, t14106: f64, t13559: f64, t2970: f64, t973: f64, t1036: f64, t13942: f64, t3047: f64, t4616: f64, t10890: f64, t14507: f64) -> (f64, f64, f64, f64, f64) {
    let t49721 = t10883 * t13969 * t14106;
    let t49732 = t973 * t2970 * t13559;
    let t49734 = t13942 * t1036;
    let t49740 = t4616 * t3047;
    let t49743 = t14507 * t10890;
    (t49721, t49732, t49734, t49740, t49743)
}
