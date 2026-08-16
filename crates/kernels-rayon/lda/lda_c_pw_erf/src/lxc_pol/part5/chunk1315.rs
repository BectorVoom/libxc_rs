//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1315/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1315(t21214: f64, t21216: f64, t21218: f64, t21222: f64, t21224: f64, t21228: f64, t21230: f64, t21234: f64, t21238: f64, t21243: f64, t21249: f64, t21251: f64, t21255: f64) -> f64 {
    let t23225 = -t21214 + t21216 + t21218 - t21222 - t21224 - t21228 - t21230 - t21234 + t21238 - t21243 - t21249 + t21251 - t21255;
    t23225
}
