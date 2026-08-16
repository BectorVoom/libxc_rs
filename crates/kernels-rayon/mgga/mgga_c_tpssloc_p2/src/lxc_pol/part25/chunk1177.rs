//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1177/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1177(t2240: f64, t2251: f64, t2250: f64, t72: f64, t79: f64, t605: f64, t9259: f64, t9240: f64, t2235: f64, t2307: f64, t641: f64, t9342: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t83778 = t2240 * t2251;
    let t83820 = t72 * t79 * t2250;
    let t83822 = t605 * t9259;
    let t83832 = t72 * t79 * t9240;
    let t83835 = t2235 * t2251;
    let t83840 = t72 * t641 * t2307;
    let t83846 = t72 * t79 * t9342;
    (t83778, t83820, t83822, t83832, t83835, t83840, t83846)
}
