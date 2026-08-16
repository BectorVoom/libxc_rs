//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 479/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk479(t50: f64, t2334: f64, t2337: f64, t52: f64, t950: f64, t2333: f64, t59: f64, zeta_threshold: f64) -> f64 {
    let t51 = t50 <= zeta_threshold;
    let t2341 = piecewise3(t51, 0.0_f64, 4.0_f64 / 9.0_f64 * t950 * t2334 + 4.0_f64 / 3.0_f64 * t52 * t2337);
    let t2343 = (t2333 + t2341) * t59;
    t2343
}
