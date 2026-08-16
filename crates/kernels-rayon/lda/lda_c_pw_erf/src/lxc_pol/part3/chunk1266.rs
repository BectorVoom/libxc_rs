//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1266/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1266(t12199: f64, t12203: f64, t12205: f64, t12207: f64, t12211: f64, t12215: f64, t12219: f64, t12223: f64, t12227: f64, t12229: f64, t12234: f64, t12239: f64, t12241: f64) -> f64 {
    let t15001 = -t12199 - t12203 + t12205 + t12207 + t12211 + t12215 + t12219 - t12223 - t12227 - t12229 - t12234 - t12239 - t12241;
    t15001
}
