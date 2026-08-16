//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3097/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3097<F: Float>(t20580: F, t58342: F, t16840: F, t20648: F, t20652: F, t58473: F, t1149: F, t12227: F, t24262: F, t12248: F, t5104: F, t6474: F) -> (F, F, F, F, F) {
    let t81631 = F::cast_from(0.2894756309764656312e3_f64) * t58342 * t20580;
    let t81633 = F::cast_from(0.96491876992155210402e2_f64) * t16840 * t20648;
    let t81635 = F::cast_from(0.1551780387578202009e4_f64) * t58473 * t20652;
    let t81638 = F::cast_from(0.57895126195293126241e3_f64) * t12227 * t24262 * t1149;
    let t81641 = F::cast_from(0.28947563097646563121e3_f64) * t12248 * t6474 * t5104;
    (t81631, t81633, t81635, t81638, t81641)
}
