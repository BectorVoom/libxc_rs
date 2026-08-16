//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 947/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk947(t4192: f64, t4207: f64, t1325: f64, t3774: f64, t3787: f64, t3779: f64, t519: f64, t1476: f64, t3742: f64, t163: f64, t169: f64, t234: f64, t2817: f64) -> (f64, f64, f64, f64, f64) {
    let t10719 = t4192 * t4207;
    let t10722 = t1325 * t3787 * t3774;
    let t10725 = t519 * t3787 * t3779;
    let t10729 = t3742 * t1476;
    let t10749 = 0.4097848972398244_f64 * t169 * t2817 * t234 * t163;
    (t10719, t10722, t10725, t10729, t10749)
}
