//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 639/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk639(t23244: f64, t925: f64, t1902: f64, t3052: f64, t5630: f64, t1882: f64, t6492: f64, t452: f64, t5750: f64, t942: f64, t23265: f64, t3204: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t26184 = t23244 * t925;
    let t26185 = t1902 * t26184;
    let t26188 = t5630 * t3052;
    let t26189 = t1902 * t26188;
    let t26192 = t1882 * t6492;
    let t26195 = t452 * t5750 * t942;
    let t26198 = t23265 * t3204;
    (t26184, t26185, t26188, t26189, t26192, t26195, t26198)
}
