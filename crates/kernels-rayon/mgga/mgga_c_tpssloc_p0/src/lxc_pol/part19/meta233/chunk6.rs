//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 946/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk946(t300: f64, t3368: f64, t1166: f64, t1155: f64, t3377: f64) -> (f64, f64, f64) {
    let t11126 = t300 * t3368;
    let t11128 = 0.17544670867903938621e1_f64 * t11126 * t1166;
    let t11129 = t3377 * t1155;
    (t11126, t11128, t11129)
}
