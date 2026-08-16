//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1253/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1253(t1041: f64, t21130: f64, t248: f64, t42592: f64, t21594: f64, t376: f64, t10422: f64, t21519: f64, t3070: f64, t135: f64, t21561: f64, t973: f64) -> (f64, f64, f64, f64) {
    let t70389 = t1041 * t248 * t42592 * t21130;
    let t70391 = t376 * t21594;
    let t70404 = t3070 * t10422 * t21519;
    let t70497 = t973 * t135 * t21561;
    (t70389, t70391, t70404, t70497)
}
