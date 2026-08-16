//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 795/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk795(t2394: f64, t463: f64, t8004: f64, t2147: f64, t322: f64, t2138: f64, t309: f64, t2131: f64, t8306: f64, t9025: f64, t8440: f64, t9029: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9150 = t8004 * t2394 * t463;
    let t9154 = t2147 * t2394 * t322;
    let t9155 = t2138 * t9154;
    let t9159 = t2147 * t2394 * t309;
    let t9160 = t2131 * t9159;
    let t9162 = t8306 * t9025;
    let t9165 = t8306 * t8440;
    let t9168 = t8306 * t9029;
    (t9150, t9154, t9155, t9159, t9160, t9162, t9165, t9168)
}
