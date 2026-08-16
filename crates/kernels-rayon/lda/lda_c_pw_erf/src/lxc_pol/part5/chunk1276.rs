//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1276/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1276(t12158: f64, t21815: f64, t571: f64, t504: f64, t7520: f64, t348: f64, t519: f64, t9351: f64, t1318: f64, t1319: f64, t549: f64, t7404: f64) -> (f64, f64, f64) {
    let t22880 = 64.0_f64 / 27.0_f64 * t571 * t12158 * t21815;
    let t22881 = t7520 * t504;
    let t22885 = 8.0_f64 / 15.0_f64 * t519 * t9351 * t22881 * t348;
    let t22889 = 8.0_f64 / 45.0_f64 * t1318 * t1319 * t7404 * t549;
    (t22880, t22885, t22889)
}
