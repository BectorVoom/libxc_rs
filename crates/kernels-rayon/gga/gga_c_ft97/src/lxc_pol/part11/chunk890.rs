//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 890/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk890(t38273: f64, t446: f64, t7793: f64, t1588: f64, t1651: f64, t7824: f64, t1882: f64, t7816: f64, t1647: f64, t1755: f64, t1564: f64, t1546: f64, t7746: f64, t89: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t38275 = t446 * t7793 * t38273;
    let t38277 = t1651 * t1588;
    let t38279 = t446 * t7824 * t38277;
    let t38281 = t1882 * t7816;
    let t38283 = t1647 * t1755;
    let t38285 = t446 * t1564 * t38283;
    let t38288 = t89 * t1546 * t7746;
    (t38275, t38277, t38279, t38281, t38283, t38285, t38288)
}
