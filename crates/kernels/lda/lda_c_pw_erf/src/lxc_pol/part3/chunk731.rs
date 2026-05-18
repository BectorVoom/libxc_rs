//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 731/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk731<F: Float>(t1971: F, t945: F, t503: F, t11: F, t188: F, t504: F, t174: F, t3540: F, t3493: F, t3530: F, t3532: F, t3534: F, t3997: F, t4600: F, t4602: F, t4605: F, t4607: F, t4612: F, t4617: F, t4622: F, t4626: F, t4630: F, t4635: F) -> (F, F, F, F, F, F) {
    let t4637 = t1971 * t945;
    let t4638 = t503 * t4637;
    let t4639 = t11 * t4638;
    let t4641 = t188 * t504;
    let t4643 = t174 * t3540 * t4641;
    let t4645 = t3997 + F::new(0.0016792592592592592) * t3530 - F::new(0.0004198148148148148) * t3534 + F::new(0.0012594444444444445) * t3493 - F::new(0.0006297222222222223) * t3532 + F::new(0.0008396296296296296) * t4600 - F::new(0.0008396296296296296) * t4602 + t4605 - F::new(0.01385388888888889) * t4607 + F::new(0.002099074074074074) * t4612 - F::new(0.007556666666666666) * t4617 + F::new(0.005037777777777778) * t4622 + F::new(0.0012594444444444445) * t4626 + F::new(0.011335) * t4630 - F::new(0.015113333333333333) * t4635 - F::new(0.003778333333333333) * t4639 + F::new(0.003778333333333333) * t4643;
    (t4637, t4638, t4639, t4641, t4643, t4645)
}
