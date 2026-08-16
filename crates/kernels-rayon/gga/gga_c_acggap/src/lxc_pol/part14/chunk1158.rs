//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1158/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1158(t2068: f64, t4680: f64, t9592: f64, t7839: f64, t9583: f64, t9582: f64, t1181: f64, t1839: f64, t360: f64, t604: f64, t27011: f64, t7351: f64, t7575: f64) -> (f64, f64, f64, f64, f64) {
    let t39985 = t2068 * t4680 * t9592;
    let t39987 = t7839 * t9583;
    let t39990 = t2068 * t4680 * t9582;
    let t39995 = t2068 * t1181 * t604 * t1839 * t360;
    let t39999 = t7575 * t1181 * t7351 * t27011;
    (t39985, t39987, t39990, t39995, t39999)
}
