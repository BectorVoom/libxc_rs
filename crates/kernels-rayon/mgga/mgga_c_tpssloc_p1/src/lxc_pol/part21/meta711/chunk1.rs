//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2547/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2547(t1036: f64, t13942: f64, t3047: f64, t4616: f64, t10890: f64, t14507: f64, t1041: f64, t13969: f64, t14188: f64, t1020: f64, t14489: f64, t248: f64, t3101: f64) -> (f64, f64, f64, f64, f64) {
    let t49734 = t13942 * t1036;
    let t49740 = t4616 * t3047;
    let t49743 = t14507 * t10890;
    let t49748 = t1041 * t13969 * t14188;
    let t49757 = t1020 * t248 * t3101 * t14489;
    (t49734, t49740, t49743, t49748, t49757)
}
