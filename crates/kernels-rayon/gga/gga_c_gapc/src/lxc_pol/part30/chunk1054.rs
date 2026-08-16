//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 1054/1331 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk1054(t1038: f64, t20594: f64, t3712: f64, t19586: f64, t9260: f64, t19510: f64, t5964: f64, t1648: f64, t1839: f64, t20198: f64, t13790: f64, t8676: f64) -> (f64, f64, f64, f64, f64) {
    let t26007 = t3712 * t1038 * t20594;
    let t26017 = t9260 * t1038 * t19586;
    let t26034 = t5964 * t19510;
    let t26102 = t1648 * t1839 * t1038 * t20198;
    let t26226 = t8676 * t13790;
    (t26007, t26017, t26034, t26102, t26226)
}
