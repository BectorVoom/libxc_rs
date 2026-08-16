//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 669/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk669(t2849: f64, t496: f64, t1113: f64, t8: f64, t1122: f64, t123: f64, t19: f64, t438: f64) -> (f64, f64, f64, f64, f64) {
    let t4290 = t496 * t2849;
    let t4298 = t8 * t1113;
    let t4328 = t1122 * t123;
    let t4355 = t19 * t1122;
    let t4356 = t123 * t438;
    (t4290, t4298, t4328, t4355, t4356)
}
