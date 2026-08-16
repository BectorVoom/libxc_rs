//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2340/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2340(t46104: f64, t7245: f64, t12571: f64, t24525: f64, t27331: f64, t9239: f64, t2110: f64, t22527: f64, t22531: f64, t22537: f64, t22546: f64, t24514: f64, t26055: f64, t27341: f64, t6492: f64, t7256: f64, t7259: f64, t7432: f64, t7978: f64, t85510: f64, t90196: f64, t90202: f64, t90205: f64) -> f64 {
    let t96025 = t46104 * t7245;
    let t96028 = t12571 * t24525;
    let t96045 = t9239 * t27331;
    let t96050 = t22537 * t7978 / 3.0_f64 + 5.0_f64 / 3.0_f64 * t96025 * t6492 + 5.0_f64 / 3.0_f64 * t96028 * t6492 + 5.0_f64 / 3.0_f64 * t27341 * t22527 + 5.0_f64 / 6.0_f64 * t27341 * t22531 + 2.0_f64 / 3.0_f64 * t90202 * t2110 + t90205 * t2110 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t26055 * t7256 + 2.0_f64 / 3.0_f64 * t26055 * t7259 + 5.0_f64 / 6.0_f64 * t85510 * t7432 - 5.0_f64 * t96045 * t22546 - 5.0_f64 * t24514 * t90196;
    t96050
}
