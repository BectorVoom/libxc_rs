//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 224/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk224(t169: f64, t633: f64, t849: f64, t612: f64, t616: f64, t626: f64, t636: f64, t653: f64, t158: f64, t89: f64, t155: f64, t573: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t851 = t849 * t169 * t633;
    let t856 = 2.0_f64 * t612;
    let t857 = 4.0_f64 / 3.0_f64 * t616;
    let t861 = t856 + t857 + 2.0_f64 * t626 + 2.0_f64 * t636 - 2.0_f64 * t653;
    let t862 = 1.0_f64 / t158;
    let t863 = t861 * t862;
    let t864 = t863 * t89;
    let t870 = 12.992782516386768_f64 * t155 * t573;
    (t851, t856, t857, t861, t862, t863, t864, t870)
}
