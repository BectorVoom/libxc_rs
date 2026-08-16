//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1052/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1052(t7841: f64, t865: f64, t2718: f64, t25049: f64, t4234: f64, t7101: f64, t1510: f64, t24269: f64, t1499: f64, t2051: f64, t23003: f64, t23026: f64, t23029: f64, t23167: f64, t23170: f64, t24246: f64, t24250: f64, t24265: f64, t25239: f64, t25243: f64, t25246: f64, t25252: f64, t25259: f64, t2617: f64, t4162: f64, t4166: f64, t7102: f64, t7104: f64, t7837: f64, t812: f64) -> (f64, f64, f64) {
    let t26581 = t7841 * t865;
    let t26582 = t2718 * t26581;
    let t26591 = 0.38381794893125283518e-1_f64 * t25049;
    let t26598 = t7101 * t4234;
    let t26608 = t24269 * t1510;
    let t26611 = -0.16449340668482264365e-1_f64 * t25239 - t812 * t26598 - 0.16449340668482264365e-1_f64 * t25243 + 0.82246703342411321825e-2_f64 * t25246 + 0.9869604401089358619e-1_f64 * t25252 + t23003 - 0.82246703342411321825e-2_f64 * t25259 + t24246 + t1499 * t7104 - 0.82246703342411321825e-2_f64 * t23026 - t23029 + t24250 - t4166 * t7102 - t2617 * t7837 - t812 * t26608 + t4162 * t2051 + t23167 + t23170 - t24265;
    (t26582, t26591, t26611)
}
