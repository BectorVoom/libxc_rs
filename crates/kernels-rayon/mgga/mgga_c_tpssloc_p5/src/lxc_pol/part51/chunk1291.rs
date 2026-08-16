//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1291/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1291(t115330: f64, t214: f64, t7191: f64, t22751: f64, t31645: f64, t31612: f64, t6883: f64, t2085: f64, t794: f64, t22892: f64, t6891: f64, t22642: f64, t22690: f64, t31618: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t115331 = 0.82246703342411321824e-2_f64 * t115330;
    let t115332 = t214 * t7191;
    let t115339 = t22751 * t31645;
    let t115341 = t6883 * t31612;
    let t115352 = t794 * t2085;
    let t115354 = t22892 * t115352 * t6891;
    let t115390 = t22642 * t22690 * t31618;
    (t115331, t115332, t115339, t115341, t115352, t115354, t115390)
}
