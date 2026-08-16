//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 940/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk940(t525: f64, t879: f64, t7932: f64, t7942: f64, t2131: f64, t2147: f64, t309: f64, t8436: f64, t2351: f64, t7924: f64, t463: f64, t8422: f64) -> (f64, f64, f64, f64, f64) {
    let t33509 = t525 * t879;
    let t33511 = t7942 * t7932 * t33509;
    let t33516 = 0.34694512752820797848e1_f64 * t2131 * t2147 * t8436 * t309;
    let t33518 = t7924 * t2351;
    let t33523 = 0.34694512752820797848e1_f64 * t2131 * t2147 * t8422 * t463;
    (t33509, t33511, t33516, t33518, t33523)
}
