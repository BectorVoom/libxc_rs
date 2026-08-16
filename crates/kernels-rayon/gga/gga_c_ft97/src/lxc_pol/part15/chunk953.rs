//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 953/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk953(t1882: f64, t20884: f64, t20939: f64, t20661: f64, t375: f64, t89: f64, t1546: f64, t20538: f64, t20534: f64, t37401: f64, t20656: f64, t20542: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t77823 = t1882 * t20884;
    let t77868 = t1882 * t20939;
    let t77914 = t89 * t375 * t20661;
    let t77917 = t89 * t1546 * t20538;
    let t77920 = t89 * t37401 * t20534;
    let t77935 = t89 * t375 * t20656;
    let t77990 = t1882 * t20542;
    (t77823, t77868, t77914, t77917, t77920, t77935, t77990)
}
