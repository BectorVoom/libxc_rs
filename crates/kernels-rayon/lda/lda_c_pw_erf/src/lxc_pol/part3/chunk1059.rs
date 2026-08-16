//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1059/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1059(t12324: f64, t12403: f64, t4488: f64, t10015: f64, t5138: f64, t10030: f64, t5152: f64, t12064: f64, t4509: f64, t108: f64, t267: f64, t564: f64) -> (f64, f64, f64, f64, f64) {
    let t12406 = 16.0_f64 / 15.0_f64 * t4488 * t12403 * t12324;
    let t12408 = 16.0_f64 / 15.0_f64 * t10015 * t5138;
    let t12409 = t10030 * t5152;
    let t12410 = 32.0_f64 / 45.0_f64 * t12409;
    let t12411 = t12064 * t4509;
    let t12412 = 32.0_f64 / 45.0_f64 * t12411;
    let t12414 = t564 * t108 * t267;
    (t12406, t12408, t12410, t12412, t12414)
}
