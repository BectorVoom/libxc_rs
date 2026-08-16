//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1263/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1263(t1863: f64, t96469: f64, t2240: f64, t5399: f64, t27948: f64, t33: f64, t55921: f64, t6489: f64, t12571: f64, t26083: f64, t1862: f64, t5392: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t96470 = t1863 * t96469;
    let t96473 = t2240 * t5399;
    let t96529 = t2240 * t33 * t27948;
    let t96532 = t55921 * t6489;
    let t96538 = t12571 * t26083;
    let t96547 = t2240 * t5392 * t1862;
    (t96470, t96473, t96529, t96532, t96538, t96547)
}
