//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 749/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk749(t5441: f64, t71: f64, t1863: f64, t5389: f64, t79: f64, t72: f64, t1410: f64, t3953: f64, t1433: f64, t1437: f64, t5445: f64, t5392: f64, t605: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t27956 = t71 * t5441;
    let t27957 = t1863 * t27956;
    let t27960 = t79 * t5389;
    let t27961 = t72 * t27960;
    let t27966 = t3953 * t1410;
    let t27971 = t1433 * t1437;
    let t27972 = t72 * t27971;
    let t27975 = t79 * t5445;
    let t27976 = t72 * t27975;
    let t27979 = t605 * t5392;
    (t27956, t27957, t27961, t27966, t27972, t27976, t27979)
}
