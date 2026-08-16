//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 738/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk738(t5143: f64, t1503: f64, t4913: f64, t541: f64, t555: f64, t1511: f64, t1639: f64, t4911: f64, t4915: f64, t114: f64, t1661: f64, t557: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5144 = 144.0_f64 * t5143;
    let t5146 = t1503 * t4913 * t541;
    let t5148 = 0.35089341735807877242e1_f64 * t555 * t5146;
    let t5149 = t1511 * t1639;
    let t5150 = 0.35089341735807877242e1_f64 * t5149;
    let t5152 = t4911 * t4913 * t4915;
    let t5154 = 0.10254018858216406658e4_f64 * t555 * t5152;
    let t5155 = t1661 * t114;
    let t5156 = t5155 * t557;
    (t5144, t5146, t5148, t5149, t5150, t5152, t5154, t5155, t5156)
}
