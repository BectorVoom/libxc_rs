//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1012/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1012(t11739: f64, t1214: f64, t248: f64, t3509: f64, t3570: f64, t3506: f64, t11159: f64, t3440: f64, t11168: f64, t1177: f64, t135: f64, t3561: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11741 = t248 * t1214 * t11739;
    let t11745 = t248 * t3570 * t3509;
    let t11746 = t3506 * t11745;
    let t11748 = t3440 * t11159;
    let t11751 = t1177 * t11168;
    let t11754 = t135 * t3561;
    (t11741, t11745, t11746, t11748, t11751, t11754)
}
