//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1001/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1001(t436: f64, t9314: f64, t1514: f64, t2628: f64, t2707: f64, t3639: f64, t10: f64, t3636: f64, t1107: f64, t1523: f64, t221: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9315 = t9314 * t436;
    let t9316 = t2628 * t1514;
    let t9319 = t3639 * t2707;
    let t9321 = t3636 * t10;
    let t9323 = 0.36622894612013090108e-3_f64 * t9321 * t1107;
    let t9324 = t1523 * t221;
    (t9315, t9316, t9319, t9321, t9323, t9324)
}
