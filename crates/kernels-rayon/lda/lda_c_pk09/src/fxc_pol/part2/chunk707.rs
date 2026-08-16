//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 707/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk707(t6464: f64, t1884: f64, t1887: f64, t533: f64, t6236: f64, t529: f64, t1803: f64, t6477: f64, t6501: f64, t6505: f64, t6522: f64, t6319: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6844 = 0.20936026574888528_f64 * t6464;
    let t6849 = t1884 * t1887;
    let t6852 = 1.0_f64 / t533 / t6236;
    let t6853 = t529 * t6852;
    let t6864 = t1803 * t6477;
    let t6873 = 0.22687409291590604_f64 * t6501;
    let t6874 = 0.22687409291590604_f64 * t6505;
    let t6878 = 0.30249879055454143_f64 * t6522;
    let t6882 = 0.04525483399593904_f64 * t6319;
    (t6844, t6849, t6853, t6864, t6873, t6874, t6878, t6882)
}
