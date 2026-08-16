//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1052/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1052(t1181: f64, t4533: f64, t604: f64, t7575: f64, t7433: f64, t8522: f64, t8518: f64, t5012: f64, t7564: f64, t8600: f64, t30546: f64, t8606: f64) -> (f64, f64, f64, f64, f64) {
    let t34708 = t7575 * t1181 * t604 * t4533;
    let t34710 = t7433 * t8522;
    let t34712 = t7433 * t8518;
    let t34716 = t7564 * t1181 * t8600 * t5012;
    let t34718 = t30546 * t8606;
    (t34708, t34710, t34712, t34716, t34718)
}
