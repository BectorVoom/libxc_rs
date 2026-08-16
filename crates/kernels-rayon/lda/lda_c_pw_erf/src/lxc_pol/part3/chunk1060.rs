//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1060/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1060(t12414: f64, t4509: f64, t1309: f64, t2065: f64, t4506: f64, t4507: f64, t3824: f64, t4508: f64, t12383: f64, t12386: f64, t12392: f64, t12395: f64, t12398: f64, t12402: f64, t12406: f64, t12408: f64, t12410: f64, t12412: f64) -> (f64, f64, f64, f64) {
    let t12416 = 16.0_f64 / 15.0_f64 * t12414 * t4509;
    let t12420 = 16.0_f64 / 15.0_f64 * t4506 * t4507 * t2065 * t1309;
    let t12423 = 8.0_f64 / 15.0_f64 * t4506 * t4508 * t3824;
    let t12424 = -t12383 - t12386 + t12392 + t12395 + t12398 + t12402 - t12406 - t12408 - t12410 + t12412 + t12416 + t12420 + t12423;
    (t12416, t12420, t12423, t12424)
}
