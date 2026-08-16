//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1019/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1019(t11543: f64, t11597: f64, t491: f64, t1235: f64, t3481: f64, t1239: f64, t68: f64, t1251: f64, t3599: f64, t225: f64, t3484: f64, t3493: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11598 = t11543 + t11597;
    let t11599 = t11598 * t491;
    let t11601 = t3481 * t1235;
    let t11604 = t1239 * t1239;
    let t11605 = 1.0_f64 / t11604;
    let t11606 = t68 * t11605;
    let t11607 = t3599 * t1251;
    let t11608 = t11606 * t11607;
    let t11613 = t3484 * t225;
    let t11616 = t11598 * t225;
    let t11620 = t1235 * t3493;
    (t11599, t11601, t11608, t11613, t11616, t11620)
}
