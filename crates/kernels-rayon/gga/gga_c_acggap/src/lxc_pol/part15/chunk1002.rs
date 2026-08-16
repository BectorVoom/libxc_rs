//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1002/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1002(t13287: f64, t31057: f64, t35284: f64, t33953: f64, t4210: f64, t13364: f64, t13299: f64, t31115: f64, t33938: f64, t7433: f64, t8779: f64, t1181: f64, t21955: f64, t30806: f64, t599: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35286 = t31057 * t13287 * t35284;
    let t35288 = t33953 * t4210;
    let t35290 = t31057 * t13364 * t35288;
    let t35301 = t31115 * t13299 * t33938;
    let t35307 = t7433 * t8779;
    let t35315 = t30806 * t1181 * t599 * t21955;
    (t35286, t35288, t35290, t35301, t35307, t35315)
}
