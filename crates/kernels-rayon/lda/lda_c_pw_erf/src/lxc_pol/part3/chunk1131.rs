//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1131/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1131(t3802: f64, t4749: f64, t519: f64, t5260: f64, t13080: f64, t1318: f64, t4784: f64, t11697: f64, t1991: f64, t11766: f64, t4829: f64, t1472: f64, t5302: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13238 = t519 * t3802 * t4749;
    let t13239 = 16.0_f64 / 45.0_f64 * t13238;
    let t13241 = t519 * t3802 * t5260;
    let t13242 = 8.0_f64 / 45.0_f64 * t13241;
    let t13244 = t1318 * t13080 * t4784;
    let t13245 = 16.0_f64 / 9.0_f64 * t13244;
    let t13248 = 16.0_f64 / 3.0_f64 * t519 * t1991 * t11697;
    let t13251 = 16.0_f64 / 5.0_f64 * t519 * t4829 * t11766;
    let t13252 = t1472 * t5302;
    (t13239, t13242, t13245, t13248, t13251, t13252)
}
