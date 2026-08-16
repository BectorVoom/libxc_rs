//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1291/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1291(t22704: f64, t31559: f64, t81326: f64, t2085: f64, t212: f64, t22642: f64, t6890: f64, t214: f64, t7191: f64, t22751: f64, t31645: f64, t31612: f64, t6883: f64) -> (f64, f64, f64, f64, f64) {
    let t115318 = t22704 * t81326 * t31559;
    let t115330 = t22642 * t212 * t2085 * t6890;
    let t115331 = 0.82246703342411321824e-2_f64 * t115330;
    let t115332 = t214 * t7191;
    let t115339 = t22751 * t31645;
    let t115341 = t6883 * t31612;
    (t115318, t115331, t115332, t115339, t115341)
}
