//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 805/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk805(t50: f64, t1789: f64, t2337: f64, t2966: f64, t52: f64, t7365: f64, t7370: f64, t59: f64, t7364: f64, zeta_threshold: f64) -> f64 {
    let t51 = t50 <= zeta_threshold;
    let t7374 = piecewise3(t51, 0.0_f64, -8.0_f64 / 27.0_f64 * t2966 * t7365 + 4.0_f64 / 3.0_f64 * t1789 * t2337 + 4.0_f64 / 3.0_f64 * t52 * t7370);
    let t7376 = (t7364 + t7374) * t59;
    t7376
}
