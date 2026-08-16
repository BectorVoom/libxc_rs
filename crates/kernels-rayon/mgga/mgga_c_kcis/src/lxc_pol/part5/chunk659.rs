//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 659/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk659(t1109: f64, t4625: f64, t345: f64, t1098: f64, t1762: f64, t1727: f64, t330: f64, t829: f64, t3274: f64, t313: f64, t934: f64, t3293: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4626 = t1109 * t4625;
    let t4627 = t345 * t4626;
    let t4630 = t1098 * t1762;
    let t4632 = t1727 * t330;
    let t4633 = t4632 * t829;
    let t4634 = t3274 * t4633;
    let t4637 = t313 * t1727;
    let t4638 = t4637 * t934;
    let t4639 = t3293 * t4638;
    (t4626, t4627, t4630, t4633, t4634, t4638, t4639)
}
