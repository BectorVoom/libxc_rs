//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 721/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk721(t24081: f64, t3424: f64, t24080: f64, t1360: f64, t378: f64, t1570: f64, t165: f64, t3188: f64, t1642: f64, t1557: f64, t376: f64, t7217: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t27416 = t24081 * t3424;
    let t27417 = t24080 * t27416;
    let t27420 = t378 * t1360;
    let t27421 = t165 * t1570;
    let t27422 = t27421 * t3188;
    let t27423 = t27420 * t27422;
    let t27426 = t1642 * t1360;
    let t27427 = t165 * t1557;
    let t27428 = t27427 * t3188;
    let t27429 = t27426 * t27428;
    let t31995 = t376 * t7217;
    (t27416, t27417, t27420, t27422, t27423, t27426, t27428, t27429, t31995)
}
