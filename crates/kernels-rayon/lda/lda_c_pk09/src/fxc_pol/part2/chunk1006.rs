//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1006/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1006(t1322: f64, t2513: f64, t5664: f64, t5679: f64, t5681: f64, t6031: f64, t6035: f64, t6043: f64, t6107: f64, t6109: f64, t6117: f64, t6120: f64, t6129: f64, t6133: f64, t6137: f64, t6152: f64, t6154: f64, t6155: f64, t6164: f64, t9770: f64) -> f64 {
    let t10928 = t5679 + 0.04115066352984959_f64 * t5681 - 4.937333717448355_f64 * t5664 * t2513 - 4.937333717448355_f64 * t1322 * t9770 + 6.496391258193384_f64 * t6107 - 6.496391258193384_f64 * t6109 - 19.489173774580152_f64 * t6117 - t6120 - t6129 + t6133 - t6137 - t6152 - t6154 + 4.738783832122567_f64 * t6155 - 22.07984838129906_f64 * t6031 - 10.80049028389238_f64 * t6035 + 10.80049028389238_f64 * t6043 - t6164;
    t10928
}
