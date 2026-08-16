//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1957/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1957(t337: f64, t5415: f64, t131: f64, t475: f64, t6218: f64, t68: f64, t7328: f64, t1730: f64, t8048: f64, t2139: f64, t6163: f64, t471: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t29584 = t5415 * t337;
    let t29585 = t29584 * t131;
    let t29593 = t6218 * t68 * t475;
    let t29594 = t7328 * t29593;
    let t29597 = t1730 * t8048;
    let t29600 = t2139 * t6163;
    let t29601 = t471 * t29600;
    (t29584, t29585, t29593, t29594, t29597, t29600, t29601)
}
