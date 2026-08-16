//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 738/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk738(t119: f64, t7633: f64, t121: f64, t861: f64, t120: f64, t1062: f64, t2270: f64, t721: f64, t168: f64, t2143: f64, t609: f64, t4030: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7634 = t7633 * t119;
    let t7635 = t121 * t861;
    let t7636 = t120 * t7635;
    let t7639 = t2270 * t1062;
    let t7640 = t7639 * t721;
    let t7642 = t2270 * t119;
    let t7647 = t168 * t2143;
    let t7648 = t7647 * t609;
    let t7649 = t121 * t7648;
    let t7650 = t4030 * t7649;
    (t7634, t7636, t7640, t7642, t7647, t7650)
}
