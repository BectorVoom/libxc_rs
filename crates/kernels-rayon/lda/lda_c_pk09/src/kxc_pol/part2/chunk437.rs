//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 437/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk437(t2262: f64, t891: f64, t890: f64, t2171: f64, t2175: f64, t2179: f64, t856: f64, t857: f64, t862: f64, t89: f64, t169: f64, t2143: f64, t844: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2263 = t891 * t2262;
    let t2264 = t890 * t2263;
    let t2269 = t856 + t857 + 2.0_f64 * t2171 + 2.0_f64 * t2175 - 2.0_f64 * t2179;
    let t2270 = t2269 * t862;
    let t2271 = t2270 * t89;
    let t2275 = t844 * t169 * t2143;
    (t2263, t2264, t2269, t2270, t2271, t2275)
}
