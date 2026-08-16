//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 947/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk947(t118: f64, t6946: f64, t2414: f64, t409: f64, t419: f64, t421: f64, t117: f64, t123: f64, t2687: f64, t315: f64, t2777: f64, t2816: f64, t3474: f64, t3481: f64, t5610: f64, t5615: f64, t5620: f64, t5622: f64, t5625: f64, t5627: f64, t5697: f64, t5698: f64, t5701: f64, t5702: f64) -> (f64, f64) {
    let t7153 = t6946 * t118;
    let t7155 = t409 * t2414;
    let t7157 = t7155 * t419 * t421;
    let t7167 = t123 * t315 * t2687 * t117;
    let t7170 = t5610 - 0.02394846802050922_f64 * t3474 + 0.031505407223141116_f64 * t7153 - 0.001975389032890948_f64 * t7157 + 0.013169260219272987_f64 * t5615 - t5620 - 0.007901556131563792_f64 * t5622 - 0.0009908551388980995_f64 * t5625 - 0.12602162889256446_f64 * t5627 - t5697 - 0.06301081444628223_f64 * t5698 + t5701 + 0.12602162889256446_f64 * t5702 + t2777 + 0.008980675507690957_f64 * t7167 + t3481 + 0.06301081444628223_f64 * t2816;
    (t7155, t7170)
}
