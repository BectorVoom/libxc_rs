//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1070/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1070(t16666: f64, t16732: f64, t16794: f64, t16869: f64, t83: f64, t99: f64, t501: f64, t5076: f64, t5169: f64, t496: f64, t5175: f64, t4882: f64, t546: f64) -> (f64, f64, f64, f64, f64) {
    let t16873 = t83 * t99 * (t16666 + t16732 + t16794 + t16869);
    let t16875 = 16.0_f64 * t501 * t5076;
    let t16876 = t501 * t5169;
    let t16878 = t496 * t5175;
    let t16880 = t4882 * t546;
    (t16873, t16875, t16876, t16878, t16880)
}
