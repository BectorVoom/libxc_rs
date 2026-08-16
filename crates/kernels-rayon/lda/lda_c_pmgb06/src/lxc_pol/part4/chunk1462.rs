//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1462/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1462(t11206: f64, t11465: f64, t1296: f64, t1297: f64, t1309: f64, t18591: f64, t18632: f64, t18641: f64, t18663: f64, t18690: f64, t18702: f64, t18723: f64, t18761: f64, t2241: f64, t2722: f64, t2730: f64, t3625: f64, t3632: f64, t384: f64, t5834: f64, t5843: f64, t5846: f64, t5849: f64, t5880: f64, t7053: f64, t7056: f64, t7060: f64, t7086: f64, t74: f64, t787: f64, t8404: f64) -> f64 {
    let t18785 = -6.0_f64 * t3632 * t2730 * t1297 - 6.0_f64 * t3632 * t2722 * t1309 + 8.0_f64 * t5834 * t5846 + 4.0_f64 * t5834 * t5849 + (t18591 + t18632 + t18641 + t18663 + t18690 + t18702 + t18723 + t18761) * t74 + 4.0_f64 * t1296 * t787 * t5880 + 4.0_f64 * t3625 * t7060 + 4.0_f64 * t1296 * t7086 * t384 + 2.0_f64 * t1296 * t2730 * t1309 + 8.0_f64 * t11206 * t2241 - 12.0_f64 * t11465 * t5843 - 12.0_f64 * t8404 * t7053 + 8.0_f64 * t3625 * t7056;
    t18785
}
