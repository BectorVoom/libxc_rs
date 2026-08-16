//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1061/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1061(t513: f64, t5137: f64, t1485: f64, t1531: f64, t1639: f64, t466: f64, t5152: f64, t1532: f64, t1661: f64, t49: f64, t4868: f64, t4871: f64) -> (f64, f64, f64, f64, f64) {
    let t16571 = t5137 * t513;
    let t16575 = 0.86748650402413918736e-1_f64 * t1531 * t1485 * t1639;
    let t16578 = 0.38025319932552508021e2_f64 * t1531 * t466 * t5152;
    let t16580 = t1661 * t49 * t1532;
    let t16582 = t4871 * t4868;
    (t16571, t16575, t16578, t16580, t16582)
}
