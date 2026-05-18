//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1126/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1126<F: Float>(t13372: F, t350: F, t4870: F, t4641: F, t4873: F, t13330: F, t13332: F, t13335: F, t13337: F, t13340: F, t13343: F, t13345: F, t13347: F, t13350: F, t13353: F, t13356: F, t13359: F, t13362: F, t13365: F, t13368: F, t13370: F) -> (F, F, F) {
    let t13373 = F::new(0.0016792592592592592) * t13372;
    let t13374 = t350 * t4870;
    let t13376 = t4641 * t4873;
    let t13378 = F::new(0.04534) * t13330 - F::new(0.011335) * t13332 - F::new(0.04534) * t13335 + F::new(0.02770777777777778) * t13337 - F::new(0.02518888888888889) * t13340 + F::new(0.04534) * t13343 - F::new(0.0012594444444444445) * t13345 - F::new(0.002099074074074074) * t13347 + F::new(0.0012594444444444445) * t13350 + F::new(0.005597530864197531) * t13353 - F::new(0.06801) * t13356 - F::new(0.007556666666666666) * t13359 - F::new(0.012594444444444445) * t13362 - F::new(0.003778333333333333) * t13365 + F::new(0.02267) * t13368 - F::new(0.005037777777777778) * t13370 + t13373 + F::new(0.003778333333333333) * t13374 - F::new(0.08312333333333333) * t13376;
    (t13374, t13376, t13378)
}
