//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1068/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1068(t1181: f64, t4263: f64, t7351: f64, t7575: f64, t1992: f64, t5606: f64, t7585: f64, t7586: f64, t4257: f64, t604: f64, t8463: f64, t4791: f64, t570: f64) -> (f64, f64, f64, f64) {
    let t34984 = t7575 * t1181 * t7351 * t4263;
    let t34990 = t7585 * t7586 * t1992 * t5606;
    let t34994 = t8463 * t1181 * t604 * t4257;
    let t34996 = t570 * t4791;
    (t34984, t34990, t34994, t34996)
}
