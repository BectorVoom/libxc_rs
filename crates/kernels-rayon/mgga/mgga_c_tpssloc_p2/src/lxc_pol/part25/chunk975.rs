//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 975/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk975(t22473: f64, t2332: f64, t2358: f64, t6530: f64, t2303: f64, t71: f64, t33: f64, t9228: f64, t2235: f64, t608: f64, t641: f64, t645: f64, t72: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22474 = t22473 * t2332;
    let t22476 = t6530 * t2358;
    let t22489 = t71 * t2303;
    let t22493 = t9228 * t33;
    let t22519 = t2235 * t608;
    let t22527 = t72 * t641 * t645;
    (t22474, t22476, t22489, t22493, t22519, t22527)
}
