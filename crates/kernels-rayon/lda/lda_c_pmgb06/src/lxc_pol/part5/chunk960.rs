//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 960/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk960(t1773: f64, t2432: f64, t1322: f64, t787: f64, t117: f64, t123: f64, t2687: f64, t740: f64, t1179: f64, t2414: f64, t419: f64, t421: f64) -> (f64, f64, f64, f64) {
    let t15121 = t1773 * t2432;
    let t15136 = t1322 * t787;
    let t15152 = t123 * t740 * t2687 * t117;
    let t15159 = t1179 * t2414 * t419 * t421;
    (t15121, t15136, t15152, t15159)
}
