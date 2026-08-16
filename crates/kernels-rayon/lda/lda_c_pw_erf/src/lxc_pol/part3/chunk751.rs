//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 751/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk751(t1976: f64, t494: f64, t4829: f64, t1325: f64, t2030: f64, t3802: f64, t519: f64, t1381: f64, t816: f64, t1308: f64, t571: f64, t2151: f64, t581: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4830 = t1976 * t494;
    let t4831 = t4829 * t4830;
    let t4833 = 32.0_f64 / 45.0_f64 * t1325 * t4831;
    let t4834 = t3802 * t2030;
    let t4836 = 16.0_f64 / 135.0_f64 * t519 * t4834;
    let t4837 = t816 * t1381;
    let t4838 = t1308 * t4837;
    let t4840 = 4.0_f64 / 45.0_f64 * t571 * t4838;
    let t4841 = t2151 * t581;
    (t4830, t4831, t4833, t4834, t4836, t4837, t4838, t4840, t4841)
}
