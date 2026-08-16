//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 712/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk712(t114: f64, t4920: f64, t1507: f64, t4913: f64, t47: f64, t58: f64, t69: f64, t82: f64, t572: f64, t66: f64) -> (f64, f64, f64, f64, f64) {
    let t4921 = t114 * t4920;
    let t4922 = t4913 * t1507;
    let t4928 = 1.0_f64 / t58 / t69 * t47 / 4.0_f64;
    let t4929 = t4928 * t82;
    let t4932 = 1.0_f64 / t66 / t572;
    (t4921, t4922, t4928, t4929, t4932)
}
