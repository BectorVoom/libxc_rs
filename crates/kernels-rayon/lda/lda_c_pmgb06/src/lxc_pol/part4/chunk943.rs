//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 943/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk943(t7065: f64, t7085: f64, t1291: f64, t1296: f64, t2238: f64, t2241: f64, t2255: f64, t2722: f64, t2730: f64, t3625: f64, t3632: f64, t378: f64, t384: f64, t5831: f64, t5834: f64, t7041: f64, t7043: f64, t7053: f64, t7056: f64, t7060: f64, t74: f64, t787: f64) -> (f64, f64) {
    let t7086 = t7065 + t7085;
    let t7088 = -t1291 * t2730 + 4.0_f64 * t1296 * t7056 + 2.0_f64 * t1296 * t7060 - 2.0_f64 * t2238 * t2255 + 4.0_f64 * t5834 * t2241 + 2.0_f64 * t3625 * t2722 - 6.0_f64 * t3632 * t7053 - t378 * t7086 - t7043 * t384 - 2.0_f64 * t5831 * t787 + t7041 * t74;
    (t7086, t7088)
}
