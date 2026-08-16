//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 503/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk503(t1391: f64, t2471: f64, t186: f64, t185: f64, t2076: f64, t813: f64, t2328: f64) -> (f64, f64, f64, f64, f64) {
    let t2472 = t1391 * t2471;
    let t2473 = t186 * t2472;
    let t2475 = 4.0_f64 / 15.0_f64 * t185 * t2473;
    let t2477 = 8.0_f64 / 15.0_f64 * t2076 * t813;
    let t2478 = -t2328;
    (t2472, t2473, t2475, t2477, t2478)
}
