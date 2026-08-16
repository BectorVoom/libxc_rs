//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1110/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1110(t4939: f64, t8676: f64, t19765: f64, t3141: f64, t20500: f64, t3712: f64, t1: f64, t424: f64, t1038: f64, t20594: f64, t19586: f64, t9260: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25842 = t8676 * t4939;
    let t25871 = t3141 * t19765;
    let t25876 = t3712 * t20500;
    let t25953 = t424 * t1;
    let t26007 = t3712 * t1038 * t20594;
    let t26017 = t9260 * t1038 * t19586;
    (t25842, t25871, t25876, t25953, t26007, t26017)
}
