//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 941/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk941(t2718: f64, t377: f64, t2722: f64, t384: f64, t2255: f64, t787: f64, t2730: f64, t3643: f64, t5785: f64, t5801: f64, t5852: f64, t5855: f64, t5860: f64, t6975: f64, t6978: f64, t7002: f64, t7005: f64, t7008: f64, t7009: f64, t7013: f64) -> (f64, f64, f64, f64, f64) {
    let t7043 = t2718 * t377;
    let t7053 = t2722 * t384;
    let t7056 = t787 * t2255;
    let t7060 = t2730 * t384;
    let t7065 = -t6975 + t6978 - t3643 - 1.532671111111111_f64 * t5852 + t5855 - t7002 + t7005 + t7008 - t5785 - 3.44851_f64 * t5860 - t7009 + t5801 + t7013;
    (t7043, t7053, t7056, t7060, t7065)
}
