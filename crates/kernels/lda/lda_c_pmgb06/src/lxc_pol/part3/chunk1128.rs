//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1128/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1128<F: Float>(t12396: F, t12547: F, t13392: F, t1832: F, t8337: F, t1476: F, t1830: F, t2923: F, t2932: F, t506: F, t839: F, t13379: F, t13382: F, t13386: F, t13390: F, t9502: F, t9503: F, t9505: F, t9522: F, t9530: F, t9532: F, t9534: F, t9537: F, t9552: F, t9554: F, t9577: F) -> (F, F, F, F, F, F) {
    let t13394 = t12396 * t13392 * t12547;
    let t13399 = t8337 * t1832;
    let t13402 = t1830 * t1476 * t2923;
    let t13405 = t1830 * t506 * t2932;
    let t13407 = t1830 * t839;
    let t13416 = F::new(0.007556666666666666) * t13379 + F::new(0.026448333333333334) * t13382 + F::new(0.006297222222222222) * t13386 - F::new(0.02267) * t13390 + F::new(0.034005) * t13394 - F::new(0.005037777777777778) * t9577 - F::new(0.0012594444444444445) * t9503 + F::new(0.003778333333333333) * t9505 - t9502 - F::new(0.061712777777777776) * t13399 - F::new(0.007556666666666666) * t13402 + F::new(0.02267) * t13405 - F::new(0.0019591358024691357) * t13407 + F::new(0.002518888888888889) * t9522 + F::new(0.002518888888888889) * t9530 + F::new(0.0016792592592592592) * t9532 - F::new(0.0006297222222222223) * t9534 - F::new(0.0006996913580246914) * t9537 - F::new(0.005877407407407408) * t9552 - F::new(0.003778333333333333) * t9554;
    (t13394, t13399, t13402, t13405, t13407, t13416)
}
