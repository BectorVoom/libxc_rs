//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1268/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1268(t11484: f64, t1691: f64, t1040: f64, t34382: f64, t11387: f64, t2993: f64, t8793: f64, t11320: f64, t1700: f64, t633: f64, t3708: f64, t9071: f64, t9256: f64) -> (f64, f64, f64, f64, f64) {
    let t35005 = t11484 * t1691;
    let t35007 = t34382 * t1040;
    let t35010 = t2993 * t11387 * t8793;
    let t35013 = t633 * t11320 * t1700;
    let t35016 = t9071 * t3708 * t9256;
    (t35005, t35007, t35010, t35013, t35016)
}
