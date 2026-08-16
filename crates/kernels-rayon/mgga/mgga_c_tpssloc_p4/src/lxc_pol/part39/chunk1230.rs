//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1230/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1230(t15621: f64, t4582: f64, t11721: f64, t3507: f64, t4977: f64, t3509: f64, t1216: f64, t15553: f64, t13969: f64, t4979: f64, t3506: f64, t4973: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15622 = t4582 * t15621;
    let t15625 = t11721 * t3507;
    let t15626 = t4977 * t15625;
    let t15627 = t4582 * t15626;
    let t15630 = t4977 * t3509;
    let t15631 = t4582 * t15630;
    let t15636 = t15553 * t1216;
    let t15637 = t4582 * t15636;
    let t15640 = t13969 * t4979;
    let t15642 = t3506 * t15640 / 1152.0_f64;
    let t15643 = t13969 * t4973;
    (t15622, t15627, t15631, t15637, t15642, t15643)
}
