//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 712/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk712(t1096: f64, t604: f64, t1181: f64, t7575: f64, t2069: f64, t4680: f64, t2068: f64, t1977: f64, t592: f64, t2066: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7576 = t604 * t1096;
    let t7577 = t1181 * t7576;
    let t7578 = t7575 * t7577;
    let t7580 = t4680 * t2069;
    let t7581 = t2068 * t7580;
    let t7583 = t592 * t1977;
    let t7584 = t7583 * t2066;
    (t7576, t7577, t7578, t7580, t7581, t7583, t7584)
}
