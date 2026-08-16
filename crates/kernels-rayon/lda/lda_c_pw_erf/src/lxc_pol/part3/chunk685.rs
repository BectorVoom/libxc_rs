//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 685/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk685(t1638: f64, t4207: f64, t3796: f64, t3801: f64, t3805: f64, t3810: f64, t3814: f64, t3816: f64, t3821: f64, t3823: f64, t4185: f64, t4188: f64, t4190: f64, t4193: f64, t4198: f64, t4201: f64, t4202: f64, t4206: f64) -> (f64, f64) {
    let t4209 = 0.011181742741110338_f64 * t1638 * t4207;
    let t4210 = -t3796 - t3801 - t4185 + 0.3246312408709453_f64 * t4188 + 0.6492624817418906_f64 * t4190 + 0.03354522822333102_f64 * t4193 + t4198 + t4201 + 0.21642082724729686_f64 * t4202 + t4206 - t4209 - t3805 + t3810 + t3814 + t3816 + t3821 - t3823;
    (t4209, t4210)
}
