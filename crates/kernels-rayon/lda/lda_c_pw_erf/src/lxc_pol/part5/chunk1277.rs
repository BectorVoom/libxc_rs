//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1277/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1277(t18474: f64, t2171: f64, t6419: f64, t6423: f64, t1325: f64, t1326: f64, t494: f64, t7655: f64, t6348: f64, t4738: f64, t6323: f64, t6327: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t22890 = 16.0_f64 / 45.0_f64 * t18474;
    let t22892 = 32.0_f64 / 27.0_f64 * t2171 * t6419;
    let t22894 = 16.0_f64 / 9.0_f64 * t2171 * t6423;
    let t22898 = 8.0_f64 / 45.0_f64 * t1325 * t1326 * t7655 * t494;
    let t22900 = 4.0_f64 / 9.0_f64 * t2171 * t6348;
    let t22902 = 8.0_f64 / 15.0_f64 * t4738 * t6323;
    let t22904 = 4.0_f64 / 15.0_f64 * t2171 * t6327;
    (t22890, t22892, t22894, t22898, t22900, t22902, t22904)
}
