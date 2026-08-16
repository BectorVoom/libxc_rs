//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 648/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk648(t3113: f64, t5691: f64, t8557: f64, t11468: f64, t25924: f64, t1339: f64, t1871: f64, t3266: f64, t3052: f64, t447: f64, t5750: f64, t925: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26318 = t5691 * t3113;
    let t26319 = t8557 * t26318;
    let t26322 = t11468 * t25924;
    let t26326 = t1871 * t1339 * t3266;
    let t26330 = t447 * t1339 * t3052;
    let t26334 = t447 * t5750 * t925;
    (t26318, t26319, t26322, t26326, t26330, t26334)
}
