//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 644/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk644(t1753: f64, t301: f64, t1532: f64, t1181: f64, t1782: f64, t3201: f64, t336: f64, t1143: f64, t1713: f64, t1788: f64, t3621: f64, t174: f64, t1795: f64) -> (f64, f64, f64, f64, f64) {
    let t6269 = t1753 * t301;
    let t6270 = t1532 * t6269;
    let t6271 = t1181 * t6270;
    let t6279 = t336 * t3201 * t1782;
    let t6283 = t336 * t1143 * t1713;
    let t6286 = t3621 * t1788;
    let t6288 = t174 * t1795;
    (t6271, t6279, t6283, t6286, t6288)
}
