//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1955/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1955(t26959: f64, t6495: f64, t26070: f64, t7032: f64, t26073: f64, t26076: f64, t23998: f64, t7435: f64, t23967: f64, t26090: f64, t23993: f64, t46104: f64, t7025: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t91890 = 32.0_f64 / 9.0_f64 * t6495 * t26959;
    let t91894 = 32.0_f64 / 9.0_f64 * t26070 * t7032;
    let t91896 = 32.0_f64 / 9.0_f64 * t26073 * t7032;
    let t91898 = 32.0_f64 / 9.0_f64 * t26076 * t7032;
    let t91900 = 32.0_f64 / 9.0_f64 * t7435 * t23998;
    let t91904 = 80.0_f64 / 9.0_f64 * t23967 * t26090;
    let t91905 = t7435 * t23993;
    let t91907 = t46104 * t7025;
    (t91890, t91894, t91896, t91898, t91900, t91904, t91905, t91907)
}
