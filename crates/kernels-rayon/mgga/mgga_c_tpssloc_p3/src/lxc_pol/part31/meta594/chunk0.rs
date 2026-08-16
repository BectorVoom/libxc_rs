//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1839/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1839(t23967: f64, t26090: f64, t23993: f64, t7435: f64, t46104: f64, t7025: f64, t26055: f64, t7032: f64, t26063: f64, t7432: f64, t84241: f64, t45844: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t91904 = 80.0_f64 / 9.0_f64 * t23967 * t26090;
    let t91905 = t7435 * t23993;
    let t91907 = t46104 * t7025;
    let t91913 = 32.0_f64 / 9.0_f64 * t26055 * t7032;
    let t91921 = 80.0_f64 / 9.0_f64 * t23967 * t26063;
    let t91922 = t84241 * t7432;
    let t91954 = t45844 * t7025;
    (t91904, t91905, t91907, t91913, t91921, t91922, t91954)
}
