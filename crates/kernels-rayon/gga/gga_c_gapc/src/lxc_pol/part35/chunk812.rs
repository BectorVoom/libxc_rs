//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 812/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk812(t9476: f64, t9477: f64, t1026: f64, t2675: f64, t2679: f64, t2682: f64, t3348: f64, t1086: f64, t2777: f64, t3371: f64, t2811: f64, t3396: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9478 = t9476 * t9477;
    let t9480 = t2675 * t1026;
    let t9481 = t9480 * t2679;
    let t9483 = t3348 * t2682;
    let t9485 = t1086 * t2777;
    let t9486 = t3371 * t9485;
    let t9488 = t3396 * t2811;
    (t9478, t9481, t9483, t9485, t9486, t9488)
}
