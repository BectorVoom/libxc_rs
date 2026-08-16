//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 630/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk630(t10085: f64, t6848: f64, t1091: f64, t24747: f64, t2599: f64, t3746: f64, t6074: f64, t14196: f64, t27757: f64, t1456: f64, t3821: f64, t729: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t28150 = t10085 * t6848;
    let t28153 = t24747 * t1091;
    let t28154 = t2599 * t28153;
    let t28157 = t6074 * t3746;
    let t28158 = t2599 * t28157;
    let t28163 = t14196 * t27757;
    let t28167 = t729 * t1456 * t3821;
    (t28150, t28153, t28154, t28157, t28158, t28163, t28167)
}
