//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1645/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1645(t1352: f64, t26421: f64, t6976: f64, t22633: f64, t22705: f64, t7736: f64, t22704: f64, t6883: f64, t7741: f64, t1998: f64, t5318: f64, t214: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t26422 = t26421 * t1352;
    let t26423 = t6976 * t26422;
    let t26424 = t22633 * t26423;
    let t26426 = t22705 * t7736;
    let t26427 = t22704 * t26426;
    let t26429 = t6883 * t7741;
    let t26432 = t1998 * t5318;
    let t26433 = t214 * t26432;
    (t26422, t26423, t26424, t26426, t26427, t26429, t26432, t26433)
}
