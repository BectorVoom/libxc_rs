//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1176/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1176(t33291: f64, t7335: f64, t33433: f64, t875: f64, t33676: f64, t33678: f64, t7073: f64, t29435: f64, t829: f64, t9895: f64, t16152: f64, t29033: f64) -> (f64, f64, f64, f64, f64) {
    let t33682 = t33291 * t7335;
    let t33685 = t33433 * t875;
    let t33687 = t7073 * t33676 * t33678 * t33685;
    let t33690 = t9895 * t829 * t29435;
    let t33692 = t29033 * t16152;
    (t33682, t33685, t33687, t33690, t33692)
}
