//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1082/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1082(t1456: f64, t7614: f64, t7433: f64, t8739: f64, t1181: f64, t2068: f64, t21342: f64, t604: f64, t1089: f64, t2079: f64, t535: f64, t7542: f64) -> (f64, f64, f64, f64) {
    let t35258 = t7614 * t1456;
    let t35260 = t7433 * t8739;
    let t35264 = t2068 * t1181 * t604 * t21342;
    let t35271 = t2079 * t1089 * t535 * t7542;
    (t35258, t35260, t35264, t35271)
}
