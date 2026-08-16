//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 860/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk860(t108: f64, t2268: f64, t2274: f64, t2329: f64, t2337: f64, t406: f64, t408: f64, t659: f64, t661: f64, t7354: f64, t7360: f64, t7365: f64, t7370: f64) -> f64 {
    let t8025 = (40.0_f64 / 27.0_f64 * t406 * t7354 + 20.0_f64 / 3.0_f64 * t2268 * t2329 + 4.0_f64 / 3.0_f64 * t659 * t7360 + 40.0_f64 / 27.0_f64 * t408 * t7365 + 20.0_f64 / 3.0_f64 * t2274 * t2337 + 4.0_f64 / 3.0_f64 * t661 * t7370) * t108;
    t8025
}
