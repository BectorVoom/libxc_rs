//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 783/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk783(t4185: f64, t4190: f64, t4193: f64, t4198: f64, t4201: f64, t4202: f64, t4206: f64, t4209: f64, t4544: f64, t4547: f64, t4583: f64, t6316: f64, t6317: f64, t6318: f64, t6319: f64, t6320: f64, t6321: f64) -> f64 {
    let t7248 = 0.1442805514981979_f64 * t4544 + 0.022363485482220676_f64 * t4547 - t6316 - t6317 - t4185 + 0.21642082724729686_f64 * t4190 + 0.011181742741110338_f64 * t4193 + t4198 + t4201 + 0.07214027574909895_f64 * t4202 + t4206 - t4209 + t6318 - t6319 + t6320 - t6321 + t4583;
    t7248
}
