//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1982/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1982(t29040: f64, t814: f64, t1509: f64, t7823: f64, t1499: f64, t16805: f64, t2051: f64, t26654: f64, t4162: f64, t4291: f64, t7839: f64, t812: f64, t829: f64, t84995: f64, t87559: f64, t92729: f64, t92738: f64, t92739: f64, t92749: f64, t92754: f64, t98546: f64, t98549: f64, t98553: f64, t98564: f64, t98571: f64) -> (f64, f64) {
    let t101694 = t814 * t29040;
    let t101698 = t7823 * t1509;
    let t101705 = 2.0_f64 * t4162 * t7839 - t87559 - t92729 - 0.3289868133696452873e-1_f64 * t98546 + 0.16449340668482264365e-1_f64 * t98549 - 0.16449340668482264365e-1_f64 * t98553 + t92738 - t92739 + 0.76763589786250567037e-1_f64 * t98564 - t812 * t101694 * t829 + t16805 * t2051 - 2.0_f64 * t4291 * t101698 * t829 + 2.0_f64 * t1499 * t26654 + t92749 + t92754 - 0.16449340668482264365e-1_f64 * t98571 - t84995;
    (t101698, t101705)
}
