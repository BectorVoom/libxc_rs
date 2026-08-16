//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1689/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1689(t19299: f64, t33: f64, t5441: f64, t71: f64, t5389: f64, t79: f64, t72: f64, t1410: f64, t3953: f64, t1433: f64, t1437: f64, t5445: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t27937 = t19299 * t33;
    let t27956 = t71 * t5441;
    let t27960 = t79 * t5389;
    let t27961 = t72 * t27960;
    let t27966 = t3953 * t1410;
    let t27971 = t1433 * t1437;
    let t27972 = t72 * t27971;
    let t27975 = t79 * t5445;
    (t27937, t27956, t27960, t27961, t27966, t27971, t27972, t27975)
}
