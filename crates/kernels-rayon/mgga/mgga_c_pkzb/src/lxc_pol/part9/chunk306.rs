//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 306/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk306(t22: f64, t28: f64, t34: f64, t38: f64, t974: f64, t984: f64, tau1: f64) -> (f64, f64) {
    let t991 = tau1 * t22;
    let t995 = -5.0_f64 / 3.0_f64 * t991 * t28 + 5.0_f64 / 3.0_f64 * t34 * t974 + 5.0_f64 / 3.0_f64 * t38 * t984;
    (t991, t995)
}
