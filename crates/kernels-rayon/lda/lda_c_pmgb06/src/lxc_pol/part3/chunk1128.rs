//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1128/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1128(t12396: f64, t12547: f64, t13392: f64, t1832: f64, t8337: f64, t1476: f64, t1830: f64, t2923: f64, t2932: f64, t506: f64, t839: f64, t13379: f64, t13382: f64, t13386: f64, t13390: f64, t9502: f64, t9503: f64, t9505: f64, t9522: f64, t9530: f64, t9532: f64, t9534: f64, t9537: f64, t9552: f64, t9554: f64, t9577: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13394 = t12396 * t13392 * t12547;
    let t13399 = t8337 * t1832;
    let t13402 = t1830 * t1476 * t2923;
    let t13405 = t1830 * t506 * t2932;
    let t13407 = t1830 * t839;
    let t13416 = 0.007556666666666666_f64 * t13379 + 0.026448333333333334_f64 * t13382 + 0.006297222222222222_f64 * t13386 - 0.02267_f64 * t13390 + 0.034005_f64 * t13394 - 0.005037777777777778_f64 * t9577 - 0.0012594444444444445_f64 * t9503 + 0.003778333333333333_f64 * t9505 - t9502 - 0.061712777777777776_f64 * t13399 - 0.007556666666666666_f64 * t13402 + 0.02267_f64 * t13405 - 0.0019591358024691357_f64 * t13407 + 0.002518888888888889_f64 * t9522 + 0.002518888888888889_f64 * t9530 + 0.0016792592592592592_f64 * t9532 - 0.0006297222222222223_f64 * t9534 - 0.0006996913580246914_f64 * t9537 - 0.005877407407407408_f64 * t9552 - 0.003778333333333333_f64 * t9554;
    (t13394, t13399, t13402, t13405, t13407, t13416)
}
