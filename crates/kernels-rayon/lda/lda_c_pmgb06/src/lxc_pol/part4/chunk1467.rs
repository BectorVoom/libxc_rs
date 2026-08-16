//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1467/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1467(t11535: f64, t1291: f64, t1296: f64, t1297: f64, t1309: f64, t18796: f64, t18804: f64, t18807: f64, t18823: f64, t18835: f64, t18842: f64, t18869: f64, t2238: f64, t2241: f64, t2255: f64, t2722: f64, t2730: f64, t3622: f64, t3632: f64, t378: f64, t384: f64, t5831: f64, t5880: f64, t7043: f64, t7086: f64, t787: f64, t8399: f64, t8413: f64) -> f64 {
    let t18876 = 24.0_f64 * t8413 * t2722 * t1297 - 24.0_f64 * t3632 * t2241 * t2255 - 2.0_f64 * t2238 * t5880 + 2.0_f64 * t8399 * t2722 - 2.0_f64 * t18796 * t384 - t7043 * t1309 - 2.0_f64 * t11535 * t787 - 4.0_f64 * t5831 * t2255 + 4.0_f64 * t1296 * t18804 + 2.0_f64 * t18807 * t1297 - t378 * (t18823 + t18835 + t18842 + t18869) - t3622 * t2730 - 2.0_f64 * t1291 * t7086;
    t18876
}
