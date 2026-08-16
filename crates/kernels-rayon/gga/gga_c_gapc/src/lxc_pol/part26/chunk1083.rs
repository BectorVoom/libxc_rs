//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 1083/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk1083(t11969: f64, t128: f64, t7333: f64, t875: f64, t966: f64, t11755: f64, t655: f64, t761: f64, t11960: f64, t28920: f64, t871: f64, t11961: f64, t29108: f64) -> (f64, f64, f64, f64) {
    let t33441 = t11969 * t7333 * t966 * t128 * t875;
    let t33444 = t761 * t655 * t11755;
    let t33447 = t871 * t11960 * t28920;
    let t33449 = t11961 * t29108;
    (t33441, t33444, t33447, t33449)
}
