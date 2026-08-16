//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 567/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk567(t3835: f64, t1062: f64, t805: f64, t721: f64, t3498: f64, t984: f64, t110: f64, t733: f64, t142: f64, t3163: f64, t174: f64, t759: f64) -> (f64, f64, f64, f64, f64) {
    let t3836 = 1.0_f64 / t3835;
    let t3843 = t805 * t1062;
    let t3844 = t3843 * t721;
    let t3847 = 5.014765625833418_f64 * t984 * t3498;
    let t3852 = t110 * t733;
    let t3853 = t3852 * t142;
    let t3855 = 7.5221484387501265_f64 * t3853 * t3163;
    let t3857 = 1.0_f64 / t759 / t174;
    (t3836, t3844, t3847, t3855, t3857)
}
