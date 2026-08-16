//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1204/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1204(t1318: f64, t13294: f64, t4868: f64, t3859: f64, t4637: f64, t519: f64, t4615: f64, t5237: f64, t11691: f64, t5256: f64, t1446: f64, t5251: f64) -> (f64, f64, f64, f64, f64) {
    let t14188 = 8.0_f64 / 9.0_f64 * t1318 * t4868 * t13294;
    let t14190 = t519 * t3859 * t4637;
    let t14191 = 16.0_f64 / 45.0_f64 * t14190;
    let t14193 = t519 * t5237 * t4615;
    let t14194 = 16.0_f64 / 9.0_f64 * t14193;
    let t14197 = 8.0_f64 / 9.0_f64 * t519 * t5256 * t11691;
    let t14199 = 32.0_f64 / 27.0_f64 * t1446 * t5251;
    (t14188, t14191, t14194, t14197, t14199)
}
