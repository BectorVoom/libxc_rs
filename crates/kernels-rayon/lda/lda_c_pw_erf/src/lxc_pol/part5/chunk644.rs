//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 644/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk644(t2062: f64, t5021: f64, t830: f64, t933: f64, t1386: f64, t2120: f64, t1234: f64, t795: f64, t1294: f64, t822: f64, t2095: f64, t803: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5022 = t5021 * t2062;
    let t5024 = t933 * t830;
    let t5039 = 16.0_f64 / 45.0_f64 * t2120 * t1386;
    let t5055 = 8.0_f64 / 45.0_f64 * t795 * t1234;
    let t5057 = 8.0_f64 / 45.0_f64 * t822 * t1294;
    let t5072 = t5021 * t2095;
    let t5076 = t933 * t803;
    (t5022, t5024, t5039, t5055, t5057, t5072, t5076)
}
