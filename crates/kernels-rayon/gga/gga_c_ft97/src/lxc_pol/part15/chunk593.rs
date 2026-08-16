//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 593/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk593(t2: f64, t9895: f64, t9802: f64, t9577: f64, t249: f64, t3051: f64, t7514: f64, t241: f64, t9567: f64, t9570: f64, t9698: f64, t259: f64, t89: f64, t9555: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9896 = t9895 * t2;
    let t9916 = t9802 * t2;
    let t9920 = t2 * t9577;
    let t9935 = 28.0_f64 / 27.0_f64 * t3051 * t249;
    let t9942 = t7514 * t2;
    let t9952 = t9567 * t241;
    let t9953 = t2 * t9570;
    let t9972 = 28.0_f64 / 81.0_f64 * t9698;
    let t9982 = 28.0_f64 / 81.0_f64 * t89 * t9555 * t259;
    (t9896, t9916, t9920, t9935, t9942, t9952, t9953, t9972, t9982)
}
