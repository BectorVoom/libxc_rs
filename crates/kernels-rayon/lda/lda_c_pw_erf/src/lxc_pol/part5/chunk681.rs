//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 681/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk681(t3004: f64, t2995: f64, t3000: f64, t3009: f64, t3016: f64, t3018: f64, t3118: f64, t3121: f64, t3125: f64, t3133: f64, t3139: f64, t3151: f64, t3155: f64, t5698: f64, t5703: f64, t5704: f64) -> f64 {
    let t6063 = 0.0002441540671567088_f64 * t3004;
    let t6064 = -t5698 + t2995 - t3000 + t6063 - t3009 - t5703 - t5704 + t3016 + t3018 + t3155 + t3118 - t3121 + t3125 + t3133 - t3139 + t3151;
    t6064
}
