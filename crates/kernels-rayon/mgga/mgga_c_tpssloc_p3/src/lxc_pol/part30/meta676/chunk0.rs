//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2107/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2107(t12571: f64, t1410: f64, t26012: f64, t7441: f64, t27971: f64, t645: f64, t72: f64, t1437: f64, t7445: f64, t1863: f64, t27975: f64, t1864: f64, t5445: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t96443 = t12571 * t1410;
    let t96454 = t7441 * t26012;
    let t96458 = t72 * t27971 * t645;
    let t96461 = t7445 * t1437;
    let t96462 = t1863 * t96461;
    let t96466 = t72 * t27975 * t645;
    let t96469 = t1864 * t5445;
    (t96443, t96454, t96458, t96462, t96466, t96469)
}
