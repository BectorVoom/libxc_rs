//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1314/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1314(t1427: f64, t5795: f64, t11636: f64, t14074: f64, t14076: f64, t14078: f64, t14083: f64, t14088: f64, t14090: f64, t15123: f64, t15125: f64, t15132: f64, t15135: f64, t15138: f64, t247: f64, t251: f64, t256: f64) -> f64 {
    let t15139 = t5795 * t1427;
    let t15140 = 0.36466666666666664_f64 * t15139;
    let t15141 = t14074 + t14076 - t14078 + t15123 / 3.0_f64 + 0.18233333333333332_f64 * t15125 + t11636 * t247 * t251 * t256 / 3.0_f64 + t15132 + 0.18233333333333332_f64 * t15135 + t15138 + t15140 - t14083 - t14088 + t14090;
    t15141
}
