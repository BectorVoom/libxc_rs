//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1238/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1238(t18025: f64, t1318: f64, t3854: f64, t7821: f64, t2146: f64, t6236: f64, t1325: f64, t1326: f64, t494: f64, t7647: f64, t1313: f64, t519: f64, t542: f64) -> (f64, f64, f64, f64, f64) {
    let t22263 = 32.0_f64 / 27.0_f64 * t18025;
    let t22265 = t1318 * t3854 * t7821;
    let t22266 = 16.0_f64 / 45.0_f64 * t22265;
    let t22267 = t2146 * t6236;
    let t22268 = 16.0_f64 / 45.0_f64 * t22267;
    let t22272 = 16.0_f64 / 15.0_f64 * t1325 * t1326 * t7647 * t494;
    let t22276 = 8.0_f64 / 15.0_f64 * t519 * t1313 * t7647 * t542;
    (t22263, t22266, t22268, t22272, t22276)
}
