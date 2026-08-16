//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1412/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1412(t28: f64, t9516: f64, t1081: f64, t2749: f64, t23788: f64, t46298: f64, t25891: f64, t9616: f64, t2745: f64, t25927: f64, t46362: f64, t46252: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t83613 = t28 * t9516;
    let t83617 = t1081 * t2749;
    let t83624 = t23788 * t46298;
    let t83627 = t25891 * t9616;
    let t83630 = t1081 * t2745;
    let t83645 = t25927 * t46362;
    let t83651 = t23788 * t46252;
    (t83613, t83617, t83624, t83627, t83630, t83645, t83651)
}
