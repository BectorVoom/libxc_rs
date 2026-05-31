//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1038/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1038<F: Float>(t12343: F, t1830: F, t453: F, t12176: F, t3090: F, t36: F, t12181: F, t1525: F, t12325: F, t12329: F, t12332: F, t12335: F, t12337: F, t12341: F, t9147: F, t9149: F, t9151: F, t9179: F, t9181: F, t9184: F, t9186: F, t9215: F, t9217: F, t9225: F) -> (F, F, F, F) {
    let t12345 = t1830 * t453 * t12343;
    let t12348 = t36 * t3090 * t12176;
    let t12351 = t1830 * t1525 * t12181;
    let t12353 = -F::cast_from(0.005037777777777778_f64) * t9147 - F::cast_from(0.003778333333333333_f64) * t9149 + F::cast_from(0.002518888888888889_f64) * t9179 + F::cast_from(0.0016792592592592592_f64) * t9181 - F::cast_from(0.0006297222222222223_f64) * t9184 - F::cast_from(0.0006996913580246914_f64) * t9186 - F::cast_from(0.005877407407407408_f64) * t9215 + F::cast_from(0.002518888888888889_f64) * t9217 - F::cast_from(0.026448333333333334_f64) * t12325 + F::cast_from(0.003778333333333333_f64) * t9151 - F::cast_from(0.0012594444444444445_f64) * t9225 + F::cast_from(0.061712777777777776_f64) * t12329 + F::cast_from(0.007556666666666666_f64) * t12332 - F::cast_from(0.02267_f64) * t12335 - F::cast_from(0.0019591358024691357_f64) * t12337 + F::cast_from(0.04534_f64) * t12341 + F::cast_from(0.06801_f64) * t12345 - F::cast_from(0.02518888888888889_f64) * t12348 - F::cast_from(0.04534_f64) * t12351;
    (t12345, t12348, t12351, t12353)
}
