//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 716/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk716(t4978: f64, t526: f64, t4913: f64, t541: f64, t4929: f64, t4934: f64, t4937: f64, t4939: f64, t4943: f64, t4945: f64, t4947: f64, t4950: f64) -> (f64, f64, f64) {
    let t4979 = t4978 * t526;
    let t4982 = t4913 * t541;
    let t4993 = -0.25319e1_f64 * t4929 + 0.16879333333333333333e1_f64 * t4934 - 0.19692555555555555555e1_f64 * t4937 - 0.93011851851851851854e0_f64 * t4939 + 0.13651666666666666667e0_f64 * t4943 - 0.27303333333333333333e0_f64 * t4945 - 0.3185388888888888889e0_f64 * t4947 - 0.36514074074074074075e0_f64 * t4950;
    (t4979, t4982, t4993)
}
