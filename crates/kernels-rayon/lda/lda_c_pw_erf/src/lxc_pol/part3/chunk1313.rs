//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1313/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1313(t256: f64, t3932: f64, t850: f64, t2260: f64, t3927: f64, t5787: f64, t652: f64, t19: f64, t4713: f64, t644: f64, t647: f64, t1432: f64, t2252: f64) -> (f64, f64, f64, f64, f64) {
    let t15123 = t850 * t3932 * t256;
    let t15125 = t2260 * t3927;
    let t15132 = t5787 * t652 * t256;
    let t15135 = t4713 * t19 * t644 * t647;
    let t15138 = t2252 * t1432 * t256;
    (t15123, t15125, t15132, t15135, t15138)
}
