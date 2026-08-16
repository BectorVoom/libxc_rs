//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1299/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1299(t11189: f64, t1845: f64, t996: f64, t11188: f64, t1587: f64, t3634: f64, t11192: f64, t2906: f64, t1504: f64, t1803: f64, t1404: f64, t997: f64) -> (f64, f64, f64, f64, f64) {
    let t35575 = t996 * t1845 * t11189;
    let t35578 = t11188 * t3634 * t1587;
    let t35580 = t2906 * t11192;
    let t35584 = t996 * t1803 * t3634 * t1504;
    let t35588 = t997 * t3634 * t1404;
    (t35575, t35578, t35580, t35584, t35588)
}
