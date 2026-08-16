//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3097/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3097(t20580: f64, t58342: f64, t16840: f64, t20648: f64, t20652: f64, t58473: f64, t1149: f64, t12227: f64, t24262: f64, t12248: f64, t5104: f64, t6474: f64) -> (f64, f64, f64, f64, f64) {
    let t81631 = 0.2894756309764656312e3_f64 * t58342 * t20580;
    let t81633 = 0.96491876992155210402e2_f64 * t16840 * t20648;
    let t81635 = 0.1551780387578202009e4_f64 * t58473 * t20652;
    let t81638 = 0.57895126195293126241e3_f64 * t12227 * t24262 * t1149;
    let t81641 = 0.28947563097646563121e3_f64 * t12248 * t6474 * t5104;
    (t81631, t81633, t81635, t81638, t81641)
}
