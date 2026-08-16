//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 640/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk640(t26198: f64, t8557: f64, t23265: f64, t3113: f64, t11854: f64, t379: f64, t447: f64, t6564: f64, t3052: f64, t5717: f64, t1909: f64, t6534: f64, t8506: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t26199 = t8557 * t26198;
    let t26202 = t23265 * t3113;
    let t26203 = t11854 * t26202;
    let t26207 = t447 * t6564 * t379;
    let t26210 = t5717 * t3052;
    let t26211 = t1909 * t26210;
    let t26214 = t8506 * t6534;
    (t26199, t26202, t26203, t26207, t26210, t26211, t26214)
}
