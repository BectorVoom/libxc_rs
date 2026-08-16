//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 890/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk890(t1901: f64, t8549: f64, t8512: f64, t2580: f64, t8470: f64, t8474: f64, t7226: f64, t8612: f64, t123: f64, t8519: f64, t734: f64, t1022: f64, t2101: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8991 = t1901 * t8549;
    let t8994 = t1901 * t8512;
    let t8997 = t2580 * t8470;
    let t9000 = t2580 * t8474;
    let t9003 = t7226 * t8612;
    let t9006 = t8519 * t123;
    let t9007 = t9006 * t734;
    let t9014 = t2101 * t1022;
    (t8991, t8994, t8997, t9000, t9003, t9006, t9007, t9014)
}
