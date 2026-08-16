//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 821/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk821(t4562: f64, t4565: f64, t4572: f64, t2425: f64, t835: f64, t6597: f64, t786: f64, t6601: f64, t813: f64, t2473: f64, t795: f64, t4592: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7530 = 4.0_f64 / 45.0_f64 * t4562;
    let t7531 = 8.0_f64 / 45.0_f64 * t4565;
    let t7532 = 8.0_f64 / 45.0_f64 * t4572;
    let t7534 = 2.0_f64 / 5.0_f64 * t2425 * t835;
    let t7536 = 4.0_f64 / 5.0_f64 * t6597 * t786;
    let t7538 = 4.0_f64 / 5.0_f64 * t6601 * t813;
    let t7540 = 4.0_f64 / 5.0_f64 * t795 * t2473;
    let t7541 = 4.0_f64 / 45.0_f64 * t4592;
    (t7530, t7531, t7532, t7534, t7536, t7538, t7540, t7541)
}
