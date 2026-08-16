//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 212/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk212(t170: f64, t748: f64, t151: f64, t573: f64, t161: f64, t650: f64, t95: f64, t120: f64, t119: f64, t174: f64, t612: f64, t616: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t750 = 0.027433775686566395_f64 * t748 * t170;
    let t752 = 1.2536914064583544_f64 * t151 * t573;
    let t754 = 3.2915558116322368_f64 * t161 * t573;
    let t755 = t95 * t650;
    let t756 = t120 * t755;
    let t757 = t119 * t756;
    let t759 = t174 * t174;
    let t760 = 1.0_f64 / t759;
    let t761 = 1.5625_f64 * t612;
    let t762 = 1.0416666666666667_f64 * t616;
    (t750, t752, t754, t755, t756, t757, t759, t760, t761, t762)
}
