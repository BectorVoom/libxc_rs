//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 711/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk711(t127: f64, t4803: f64, t500: f64, t78: f64, t1503: f64, t4913: f64, t541: f64, t555: f64, t1511: f64, t1639: f64, t4911: f64, t4915: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5141 = 24.0_f64 * t4803 * t127;
    let t5142 = t78 * t500;
    let t5143 = t5142 * t127;
    let t5144 = 144.0_f64 * t5143;
    let t5146 = t1503 * t4913 * t541;
    let t5148 = 0.35089341735807877242e1_f64 * t555 * t5146;
    let t5149 = t1511 * t1639;
    let t5152 = t4911 * t4913 * t4915;
    (t5141, t5142, t5143, t5144, t5146, t5148, t5149, t5152)
}
