//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 799/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk799(t159: f64, t285: f64, t7337: f64, t2700: f64, t2703: f64, t2709: f64, t2712: f64, t2739: f64, t7323: f64, t7324: f64, t7325: f64, t7326: f64, t7327: f64, t7328: f64, t7329: f64) -> (f64, f64) {
    let t7339 = t7337 * t159 * t285;
    let t7349 = -t7323 + t2700 + t2703 - t2709 - t2712 + t7324 - t7325 - t7326 - t2739 + t7327 + t7328 + t7329;
    (t7339, t7349)
}
