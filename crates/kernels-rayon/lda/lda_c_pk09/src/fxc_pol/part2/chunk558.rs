//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 558/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk558(t119: f64, t3557: f64, t748: f64, t827: f64, t609: f64, t873: f64, t96: f64, t839: f64, t1067: f64, t864: f64, t3330: f64, t3332: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3558 = t119 * t3557;
    let t3559 = 24.533164868110067_f64 * t3558;
    let t3568 = t748 * t827;
    let t3577 = t96 * t873 * t609;
    let t3578 = t839 * t3577;
    let t3580 = t864 * t1067;
    let t3598 = 2.0_f64 * t3330;
    let t3599 = 8.0_f64 / 3.0_f64 * t3332;
    (t3558, t3559, t3568, t3578, t3580, t3598, t3599)
}
