//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1203/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1203(t30780: f64, t38956: f64, t336: f64, t5674: f64, t578: f64, t599: f64, t1773: f64, t2060: f64, t2061: f64, t6388: f64, t7450: f64, t7815: f64) -> (f64, f64, f64, f64) {
    let t40569 = t30780 * t38956;
    let t40573 = t578 * t336 * t599 * t5674;
    let t40576 = t2060 * t1773 * t2061;
    let t40579 = t7450 * t7815 * t6388;
    (t40569, t40573, t40576, t40579)
}
