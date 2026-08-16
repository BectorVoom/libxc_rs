//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 257/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk257(t1146: f64, t8: f64, t1137: f64, t1139: f64, t1143: f64, t240: f64, t5: f64, t252: f64) -> (f64, f64, f64) {
    let t1147 = t8 * t1146;
    let t1150 = -2.0004184593989263_f64 * t1137 - 8.223552159732785_f64 * t1139 + 5.687617677680484_f64 * t5 - 0.2360201854237762_f64 * t1143 - 9.157473255573062e-05_f64 * t240 * t1147;
    let t1151 = t1150 * t252;
    (t1147, t1150, t1151)
}
