//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 790/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk790(t4041: f64, t5179: f64, t5186: f64, t5190: f64, t5192: f64, t5194: f64, t5198: f64, t5200: f64, t6785: f64, t6786: f64, t6790: f64, t6792: f64, t6847: f64, t6849: f64, t6853: f64, t6858: f64, t6860: f64) -> f64 {
    let t7270 = -t6785 + t5179 - t6786 + t6790 + t4041 + t6792 - t5186 + t5190 + t5192 + t5194 - t5198 + t5200 - t6847 - t6849 + t6853 - t6858 - t6860;
    t7270
}
