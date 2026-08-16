//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 803/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk803(t43: f64, t1781: f64, t2329: f64, t2953: f64, t47: f64, t7354: f64, t7360: f64, t2334: f64, t743: f64, zeta_threshold: f64) -> (f64, f64) {
    let t44 = t43 <= zeta_threshold;
    let t7364 = piecewise3(t44, 0.0_f64, -8.0_f64 / 27.0_f64 * t2953 * t7354 + 4.0_f64 / 3.0_f64 * t1781 * t2329 + 4.0_f64 / 3.0_f64 * t47 * t7360);
    let t7365 = t2334 * t743;
    (t7364, t7365)
}
