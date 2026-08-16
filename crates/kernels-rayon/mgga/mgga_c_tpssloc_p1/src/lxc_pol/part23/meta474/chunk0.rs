//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1418/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1418(t136: f64, t3297: f64, t78031: f64, t78039: f64, t1113: f64, t78047: f64, t78043: f64, t1100: f64, t78077: f64, t3287: f64, t78025: f64, t11219: f64, t78035: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t78084 = t136 * t3297 * t78031;
    let t78087 = t136 * t3297 * t78039;
    let t78090 = t136 * t1113 * t78047;
    let t78093 = t136 * t1113 * t78043;
    let t78095 = t1100 * t78077;
    let t78097 = t3287 * t78025;
    let t78100 = t136 * t11219 * t78035;
    (t78084, t78087, t78090, t78093, t78095, t78097, t78100)
}
