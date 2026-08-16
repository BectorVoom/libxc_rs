//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1292/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1292(t2085: f64, t794: f64, t22892: f64, t6891: f64, t22642: f64, t22690: f64, t31618: f64, t22751: f64, t31620: f64, t552: f64, t7191: f64, t22893: f64, t31619: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t115352 = t794 * t2085;
    let t115354 = t22892 * t115352 * t6891;
    let t115390 = t22642 * t22690 * t31618;
    let t115391 = 0.82246703342411321824e-2_f64 * t115390;
    let t115397 = t22751 * t31620;
    let t115399 = t552 * t7191;
    let t115409 = t22892 * t22893 * t31619;
    (t115352, t115354, t115391, t115397, t115399, t115409)
}
