//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1235/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1235(t8985: f64, t9003: f64, t9005: f64, t9009: f64, t9011: f64, t9015: f64, t9017: f64, t9094: f64, t9096: f64, t9098: f64, t9100: f64, t9104: f64, t9110: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14566 = 0.9743416666666667_f64 * t8985;
    let t14567 = 4.5469277777777775_f64 * t9003;
    let t14568 = 1.9486833333333333_f64 * t9005;
    let t14569 = 2.923025_f64 * t9009;
    let t14570 = 3.8973666666666666_f64 * t9011;
    let t14571 = 1.9486833333333333_f64 * t9015;
    let t14572 = 3.8973666666666666_f64 * t9017;
    let t14579 = t14566 + t14567 - t14568 + t14569 + t14570 - t14571 - t14572 - 5.172765_f64 * t9094 + 5.364348888888889_f64 * t9096 - 2.2990066666666666_f64 * t9098 + 0.5747516666666667_f64 * t9100 + 6.89702_f64 * t9104 + 6.89702_f64 * t9110;
    (t14566, t14567, t14568, t14569, t14570, t14571, t14572, t14579)
}
