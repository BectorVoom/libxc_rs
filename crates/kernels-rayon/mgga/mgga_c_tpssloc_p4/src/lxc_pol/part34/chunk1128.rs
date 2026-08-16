//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1128/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1128(t1811: f64, t80775: f64, t7709: f64, t80766: f64, t22724: f64, t26344: f64, t22643: f64, t7691: f64, t81195: f64, t22573: f64, t7684: f64, t23993: f64, t7435: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t91398 = t80775 * t1811;
    let t91400 = t80766 * t7709;
    let t91531 = t22724 * t26344;
    let t91548 = t81195 * t22643 * t7691;
    let t91655 = t7684 * t22573;
    let t91905 = t7435 * t23993;
    (t91398, t91400, t91531, t91548, t91655, t91905)
}
