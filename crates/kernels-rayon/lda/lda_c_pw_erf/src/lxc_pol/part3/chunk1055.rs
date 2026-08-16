//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1055/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1055(t3704: f64, t4487: f64, t34: f64, t348: f64, t542: f64, t4494: f64, t12329: f64, t4502: f64, t12334: f64, t4488: f64, t4501: f64, t4495: f64, t945: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12362 = t4487 * t3704;
    let t12364 = t34 * t542 * t348;
    let t12367 = 32.0_f64 / 15.0_f64 * t12362 * t4494 * t12364;
    let t12369 = 8.0_f64 / 9.0_f64 * t12329 * t4502;
    let t12372 = 4.0_f64 / 9.0_f64 * t4488 * t4501 * t12334;
    let t12373 = t4495 * t945;
    (t12362, t12364, t12367, t12369, t12372, t12373)
}
