//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1038/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1038(t12343: f64, t1830: f64, t453: f64, t12176: f64, t3090: f64, t36: f64, t12181: f64, t1525: f64, t12325: f64, t12329: f64, t12332: f64, t12335: f64, t12337: f64, t12341: f64, t9147: f64, t9149: f64, t9151: f64, t9179: f64, t9181: f64, t9184: f64, t9186: f64, t9215: f64, t9217: f64, t9225: f64) -> (f64, f64, f64, f64) {
    let t12345 = t1830 * t453 * t12343;
    let t12348 = t36 * t3090 * t12176;
    let t12351 = t1830 * t1525 * t12181;
    let t12353 = -0.005037777777777778_f64 * t9147 - 0.003778333333333333_f64 * t9149 + 0.002518888888888889_f64 * t9179 + 0.0016792592592592592_f64 * t9181 - 0.0006297222222222223_f64 * t9184 - 0.0006996913580246914_f64 * t9186 - 0.005877407407407408_f64 * t9215 + 0.002518888888888889_f64 * t9217 - 0.026448333333333334_f64 * t12325 + 0.003778333333333333_f64 * t9151 - 0.0012594444444444445_f64 * t9225 + 0.061712777777777776_f64 * t12329 + 0.007556666666666666_f64 * t12332 - 0.02267_f64 * t12335 - 0.0019591358024691357_f64 * t12337 + 0.04534_f64 * t12341 + 0.06801_f64 * t12345 - 0.02518888888888889_f64 * t12348 - 0.04534_f64 * t12351;
    (t12345, t12348, t12351, t12353)
}
