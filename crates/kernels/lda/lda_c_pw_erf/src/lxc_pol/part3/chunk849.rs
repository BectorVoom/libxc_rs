//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 849/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk849<F: Float>(t143: F, t145: F, t2767: F, t279: F, t2880: F, t2897: F, t2903: F, t2906: F, t2932: F, t2935: F, t2937: F, t296: F, t3203: F, t405: F, t4122: F, t4125: F, t4129: F, t4132: F, t4136: F, t4140: F, t4144: F, t5548: F, t5718: F, t5745: F, t5750: F, t5779: F, t5783: F, t5902: F, t5920: F, t5924: F, t5925: F, t5933: F, t5941: F) -> F {
    let t5942 = F::cast_from(0.39633663517353707_f64) * t3203 + (F::cast_from(0.31995040645307626_f64) * t5745 + F::cast_from(0.05332506774217938_f64) * t145 * t5718 - t5750 - F::cast_from(0.10665013548435875_f64) * t2937 + F::cast_from(0.6399008129061525_f64) * t2935 + t2880 - F::cast_from(0.06367133154935875_f64) * t2906 - t2932 + t2897 - F::cast_from(0.031835665774679375_f64) * t2903 + t5779) * t296 - F::new(6.0) * t5783 * t2767 + (t5902 + t5920) * t279 + F::new(12.0) * t5924 * t5925 + F::new(3.0) * t405 * t143 * t5548 - F::cast_from(1.82185769317151e-05_f64) * t5933 - F::cast_from(0.0002905674151788692_f64) * t4122 - F::cast_from(0.0011622696607154768_f64) * t4125 - t4129 + F::cast_from(0.002711962541669446_f64) * t4132 + t4136 - t4140 - t4144 - t5941;
    t5942
}
