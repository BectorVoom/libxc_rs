//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1288/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1288(t2260: f64, t3936: f64, t5788: f64, t656: f64, t12955: f64, t12959: f64, t12962: f64, t12966: f64, t12971: f64, t12975: f64, t12979: f64, t12982: f64, t12985: f64, t12988: f64, t12991: f64) -> f64 {
    let t15060 = t2260 * t3936;
    let t15062 = t5788 * t656;
    let t15064 = t12955 + t12959 + t12962 - t12966 + t12971 - 0.013506172839506173_f64 * t15060 + 2.0_f64 / 3.0_f64 * t15062 - t12975 + t12979 - t12982 + t12985 + t12988 + t12991;
    t15064
}
