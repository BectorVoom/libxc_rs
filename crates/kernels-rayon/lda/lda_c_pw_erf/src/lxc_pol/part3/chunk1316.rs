//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1316/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1316(t14174: f64, t14176: f64, t14178: f64, t14183: f64, t14188: f64, t14191: f64, t14194: f64, t14197: f64, t14199: f64, t14203: f64, t14208: f64, t14210: f64, t14212: f64, t14216: f64) -> f64 {
    let t15156 = -t14174 + t14176 + t14178 + t14183 - t14188 - t14191 - t14194 + t14197 + t14199 + t14203 + t14208 + t14210 - t14212 - t14216;
    t15156
}
