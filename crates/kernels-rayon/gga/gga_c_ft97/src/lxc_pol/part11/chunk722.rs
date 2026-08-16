//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 722/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk722(t255: f64, t9802: f64, t3892: f64, t9797: f64, t2347: f64, t761: f64, t9792: f64, t3891: f64, t1882: f64, t2471: f64, t2459: f64, t729: f64, t773: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9803 = t9802 * t255;
    let t9804 = t3892 * t9797;
    let t9805 = t9803 * t9804;
    let t9808 = t761 * t2347;
    let t9809 = t9808 * t9792;
    let t9810 = t3891 * t9809;
    let t9813 = t1882 * t2471;
    let t9816 = t729 * t773 * t2459;
    (t9803, t9804, t9805, t9808, t9809, t9810, t9813, t9816)
}
