//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 830/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk830(t5862: f64, t5879: f64, t1291: f64, t1296: f64, t1297: f64, t1309: f64, t2238: f64, t2241: f64, t2255: f64, t3622: f64, t3625: f64, t3632: f64, t378: f64, t384: f64, t5829: f64, t5831: f64, t5834: f64, t5843: f64, t5846: f64, t5849: f64, t74: f64, t787: f64) -> (f64, f64) {
    let t5880 = t5862 + t5879;
    let t5882 = -2.0_f64 * t1291 * t2255 + 4.0_f64 * t1296 * t5846 + 2.0_f64 * t1296 * t5849 + 2.0_f64 * t5834 * t1297 - t2238 * t1309 + 4.0_f64 * t3625 * t2241 - t3622 * t787 - 6.0_f64 * t3632 * t5843 - t378 * t5880 - 2.0_f64 * t5831 * t384 + t5829 * t74;
    (t5880, t5882)
}
