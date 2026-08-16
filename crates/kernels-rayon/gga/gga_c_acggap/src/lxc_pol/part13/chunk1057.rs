//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1057/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1057(t1181: f64, t5116: f64, t7351: f64, t7564: f64, t1350: f64, t1992: f64, t30147: f64, t7586: f64, t142: f64, t4495: f64, t7436: f64, t4479: f64, t8888: f64) -> (f64, f64, f64, f64) {
    let t34522 = t7564 * t1181 * t7351 * t5116;
    let t34526 = t30147 * t7586 * t1992 * t1350;
    let t34529 = t7436 * t142 * t4495;
    let t34532 = t8888 * t142 * t4479;
    (t34522, t34526, t34529, t34532)
}
