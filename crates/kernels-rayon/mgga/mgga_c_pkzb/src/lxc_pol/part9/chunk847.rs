//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 847/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk847(t2320: f64, t6121: f64, t6122: f64, t898: f64, t6087: f64, t6090: f64, t6093: f64, t6108: f64, t378: f64, t237: f64, t2192: f64, t2235: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6124 = t6121 * t6122 * t2320;
    let t6126 = 0.10389515463408878255e3_f64 * t898 * t6124;
    let t6127 = 0.28842592592592592592e-1_f64 * t6087;
    let t6131 = -t6127 + 0.37083333333333333334e-1_f64 * t6090 - 0.278125e-1_f64 * t6093 + 0.278125e-1_f64 * t6108;
    let t6132 = t6131 * t378;
    let t6134 = 0.19751673498613801407e-1_f64 * t237 * t6132;
    let t6136 = 3.0_f64 * t2192 * t2235;
    (t6124, t6126, t6127, t6131, t6132, t6134, t6136)
}
