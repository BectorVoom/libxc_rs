//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1032/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1032(t1165: f64, t4550: f64, t7351: f64, t7575: f64, t1530: f64, t1535: f64, t30539: f64, t4762: f64, t7564: f64, t8600: f64, t30308: f64, t30310: f64) -> (f64, f64, f64, f64, f64) {
    let t34201 = t7575 * t1165 * t7351 * t4550;
    let t34204 = t1530 * t30539 * t1535;
    let t34208 = t7564 * t1165 * t8600 * t4762;
    let t34210 = 77.0_f64 / 288.0_f64 * t30308;
    let t34211 = 77.0_f64 / 864.0_f64 * t30310;
    (t34201, t34204, t34208, t34210, t34211)
}
