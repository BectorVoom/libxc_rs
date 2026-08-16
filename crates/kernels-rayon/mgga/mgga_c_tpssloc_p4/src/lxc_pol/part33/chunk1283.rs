//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1283/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1283(t23110: f64, t23185: f64, t28418: f64, t23168: f64, t28330: f64, t28406: f64, t814: f64, t234: f64, t5631: f64, t5593: f64, t81749: f64, t22690: f64, t23122: f64, t5544: f64, t841: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t98549 = t23185 * t23110 * t28418;
    let t98564 = t23168 * t28330;
    let t98592 = t814 * t28406;
    let t98598 = t234 * t5631;
    let t98618 = t81749 * t5593;
    let t98647 = t23122 * t22690 * t841 * t5544;
    (t98549, t98564, t98592, t98598, t98618, t98647)
}
