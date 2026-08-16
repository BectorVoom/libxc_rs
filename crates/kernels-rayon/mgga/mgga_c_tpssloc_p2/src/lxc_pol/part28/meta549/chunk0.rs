//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1819/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1819(t1864: f64, t2241: f64, t608: f64, t9231: f64, t645: f64, t6509: f64, t2307: f64, t2240: f64, t2251: f64, t22573: f64, t6875: f64, t24486: f64, t576: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t83718 = t1864 * t2241;
    let t83722 = t9231 * t608;
    let t83728 = t6509 * t645;
    let t83737 = t1864 * t2307;
    let t83778 = t2240 * t2251;
    let t83886 = t6875 * t22573;
    let t84031 = t576 * t24486;
    (t83718, t83722, t83728, t83737, t83778, t83886, t84031)
}
