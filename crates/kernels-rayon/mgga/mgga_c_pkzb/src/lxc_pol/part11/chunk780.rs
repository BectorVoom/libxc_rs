//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 780/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk780(t7335: f64, t1861: f64, t2759: f64, t1873: f64, t1073: f64, t5511: f64, t5547: f64, t218: f64, t2774: f64, t675: f64, t2778: f64, t1070: f64, t1898: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7359 = 2.0_f64 / 3.0_f64 * t7335;
    let t7365 = t1861 * t2759;
    let t7370 = t1873 * t2759;
    let t7375 = t5511 * t1073;
    let t7378 = t5547 * t1073;
    let t7386 = t218 * t675 * t2774;
    let t7387 = 0.41678e0_f64 * t7386;
    let t7389 = t218 * t675 * t2778;
    let t7390 = 0.41678e0_f64 * t7389;
    let t7411 = t1070 * t1898;
    (t7359, t7365, t7370, t7375, t7378, t7386, t7387, t7389, t7390, t7411)
}
