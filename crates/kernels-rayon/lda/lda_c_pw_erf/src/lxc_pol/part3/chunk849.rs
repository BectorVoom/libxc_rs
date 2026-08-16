//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 849/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk849(t143: f64, t145: f64, t2767: f64, t279: f64, t2880: f64, t2897: f64, t2903: f64, t2906: f64, t2932: f64, t2935: f64, t2937: f64, t296: f64, t3203: f64, t405: f64, t4122: f64, t4125: f64, t4129: f64, t4132: f64, t4136: f64, t4140: f64, t4144: f64, t5548: f64, t5718: f64, t5745: f64, t5750: f64, t5779: f64, t5783: f64, t5902: f64, t5920: f64, t5924: f64, t5925: f64, t5933: f64, t5941: f64) -> f64 {
    let t5942 = 0.39633663517353707_f64 * t3203 + (0.31995040645307626_f64 * t5745 + 0.05332506774217938_f64 * t145 * t5718 - t5750 - 0.10665013548435875_f64 * t2937 + 0.6399008129061525_f64 * t2935 + t2880 - 0.06367133154935875_f64 * t2906 - t2932 + t2897 - 0.031835665774679375_f64 * t2903 + t5779) * t296 - 6.0_f64 * t5783 * t2767 + (t5902 + t5920) * t279 + 12.0_f64 * t5924 * t5925 + 3.0_f64 * t405 * t143 * t5548 - 1.82185769317151e-05_f64 * t5933 - 0.0002905674151788692_f64 * t4122 - 0.0011622696607154768_f64 * t4125 - t4129 + 0.002711962541669446_f64 * t4132 + t4136 - t4140 - t4144 - t5941;
    t5942
}
