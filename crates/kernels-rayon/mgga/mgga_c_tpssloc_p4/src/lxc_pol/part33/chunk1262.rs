//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1262/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1262(t2022: f64, t6483: f64, t1864: f64, t5389: f64, t1863: f64, t12571: f64, t1410: f64, t26012: f64, t7441: f64, t1437: f64, t7445: f64, t5445: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t96348 = t2022 * t6483;
    let t96425 = t1864 * t5389;
    let t96426 = t1863 * t96425;
    let t96443 = t12571 * t1410;
    let t96454 = t7441 * t26012;
    let t96461 = t7445 * t1437;
    let t96462 = t1863 * t96461;
    let t96469 = t1864 * t5445;
    (t96348, t96426, t96443, t96454, t96462, t96469)
}
