//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1232/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1232(t3532: f64, t7370: f64, t2765: f64, t9164: f64, t10806: f64, t1873: f64, t667: f64, t10800: f64, t17432: f64, t2759: f64, t9137: f64, t7365: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t30314 = t7370 * t3532;
    let t30316 = t2765 * t9164;
    let t30319 = t1873 * t10806 * t667;
    let t30322 = t17432 * t10800 * t667;
    let t30324 = t9137 * t2759;
    let t30326 = t7365 * t3532;
    (t30314, t30316, t30319, t30322, t30324, t30326)
}
