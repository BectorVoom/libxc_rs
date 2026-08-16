//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 780/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk780(t4468: f64, t4470: f64, t6161: f64, t6192: f64, t6197: f64, t6200: f64, t6202: f64, t6204: f64, t6207: f64, t6211: f64, t6213: f64, t6217: f64, t6219: f64, t6222: f64, t6224: f64, t6228: f64) -> f64 {
    let t7238 = -t6161 + t4468 + t4470 - t6192 + t6197 + t6200 - t6202 + t6204 + t6207 + t6211 + t6213 + t6217 + t6219 + t6222 - t6224 + t6228;
    t7238
}
