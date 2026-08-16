//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1057/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1057(t16559: f64, t16540: f64, t16557: f64, t555: f64, t1518: f64, t1527: f64, t1599: f64, t513: f64, t5137: f64, t1485: f64, t1531: f64, t1639: f64) -> (f64, f64, f64, f64, f64) {
    let t16560 = 1.0_f64 / t16559;
    let t16563 = 0.91082604192152556044e5_f64 * t555 * t16557 * t16540 * t16560;
    let t16569 = 36.0_f64 * t1599 * t1518 * t1527;
    let t16571 = t5137 * t513;
    let t16575 = 0.86748650402413918736e-1_f64 * t1531 * t1485 * t1639;
    (t16560, t16563, t16569, t16571, t16575)
}
