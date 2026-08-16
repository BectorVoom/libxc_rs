//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 917/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk917(t7291: f64, t883: f64, t5641: f64, t9805: f64, t2365: f64, t7292: f64, t6111: f64, t3295: f64, t826: f64, t825: f64, t2021: f64, t2672: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9806 = t883 * t7291;
    let t9807 = t5641 * t9806;
    let t9809 = 0.11502877786176224903e1_f64 * t9805 * t9807;
    let t9810 = t2365 * t7292;
    let t9812 = 0.59584149919750711116e-1_f64 * t6111 * t9810;
    let t9813 = t826 * t3295;
    let t9814 = t825 * t9813;
    let t9820 = t2021 * t2672;
    (t9807, t9809, t9810, t9812, t9813, t9814, t9820)
}
