//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 490/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk490(t1313: f64, t2396: f64, t519: f64, t811: f64, t209: f64, t184: f64) -> (f64, f64, f64, f64, f64) {
    let t2397 = t1313 * t2396;
    let t2399 = 8.0_f64 / 45.0_f64 * t519 * t2397;
    let t2400 = t811 * t811;
    let t2401 = t2400 * t209;
    let t2402 = t2401 * t184;
    (t2397, t2399, t2400, t2401, t2402)
}
