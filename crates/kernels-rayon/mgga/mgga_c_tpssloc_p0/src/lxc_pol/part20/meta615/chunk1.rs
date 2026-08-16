//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2217/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2217(t12606: f64, t607: f64, t12648: f64, t12649: f64, t12652: f64, t12653: f64, t12661: f64, t12709: f64, t1434: f64, t2252: f64, t31: f64, t4018: f64, t45872: f64, t45993: f64, t45997: f64, t628: f64, t642: f64, t65: f64, t80: f64, t9263: f64) -> (f64, f64) {
    let t46006 = t607 * t12606;
    let t46022 = t12709 * t642 / 8.0_f64 - t9263 * t1434 / 4.0_f64 - t2252 * t4018 / 4.0_f64 - t45993 * t65 * t80 / 12.0_f64 - t45997 * t65 * t80 / 4.0_f64 - t12648 * t628 * t80 / 4.0_f64 - t12649 * t642 / 4.0_f64 - t46006 * t65 * t80 / 4.0_f64 - t12652 * t628 * t80 / 2.0_f64 - t12653 * t642 / 2.0_f64 - t31 * t45872 * t65 * t80 / 12.0_f64 - t12661 * t628 * t80 / 4.0_f64;
    (t46006, t46022)
}
