//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1610/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1610(t11720: f64, t11721: f64, t1214: f64, t248: f64, t11717: f64, t3503: f64, t11713: f64) -> (f64, f64, f64, f64) {
    let t11722 = t11720 * t11721;
    let t11724 = t248 * t1214 * t11722;
    let t11727 = t3503 * t11717;
    let t11728 = t11713 * t11727;
    (t11722, t11724, t11727, t11728)
}
