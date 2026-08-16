//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 891/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk891(t2100: f64, t30567: f64, t2104: f64, t7630: f64, t1035: f64, t1979: f64, t355: f64, t864: f64, t368: f64, t7458: f64, t7709: f64, t7799: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t30568 = t30567 * t2100;
    let t30569 = 0.56606566121287473723e-2_f64 * t30568;
    let t30570 = t7630 * t2104;
    let t30572 = t1035 * t1979;
    let t30573 = t355 * t864;
    let t30576 = t30572 * t7458 * t368 * t30573;
    let t30577 = 0.42874018118069736972e-3_f64 * t30576;
    let t30582 = t7799 * t7709;
    (t30569, t30570, t30572, t30573, t30577, t30582)
}
