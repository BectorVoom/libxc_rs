//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 795/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk795(t2535: f64, t919: f64, t9497: f64, t1084: f64, t3717: f64, t2657: f64, t2660: f64, t9019: f64, t2721: f64, t3103: f64, t2255: f64, t2636: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9499 = t2535 * t919 * t9497;
    let t9501 = t1084 * t3717;
    let t9502 = t9501 * t2657;
    let t9504 = t2660 * t9019;
    let t9505 = t9504 * t2657;
    let t9507 = t2721 * t3103;
    let t9508 = t2636 * t2255;
    (t9499, t9501, t9502, t9504, t9505, t9507, t9508)
}
