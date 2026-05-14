//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 917/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk917<F: Float>(t1830: F, t2969: F, t453: F, t810: F, t3010: F, t4644: F, t1525: F, t36: F, t1069: F, t4654: F, t12176: F, t3090: F, t12181: F, t12325: F, t12329: F, t12332: F, t9147: F, t9149: F, t9151: F, t9179: F, t9181: F, t9184: F, t9186: F, t9215: F, t9217: F, t9225: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12335 = t1830 * t453 * t2969;
    let t12337 = t1830 * t810;
    let t12339 = t4644 * t3010;
    let t12341 = t36 * t1525 * t12339;
    let t12343 = t4654 * t1069;
    let t12345 = t1830 * t453 * t12343;
    let t12348 = t36 * t3090 * t12176;
    let t12351 = t1830 * t1525 * t12181;
    let t12353 = -0.005037777777777778 * t9147 - 0.003778333333333333 * t9149 + 0.002518888888888889 * t9179 + 0.0016792592592592592 * t9181 - 0.0006297222222222223 * t9184 - 0.0006996913580246914 * t9186 - 0.005877407407407408 * t9215 + 0.002518888888888889 * t9217 - 0.026448333333333334 * t12325 + 0.003778333333333333 * t9151 - 0.0012594444444444445 * t9225 + 0.061712777777777776 * t12329 + 0.007556666666666666 * t12332 - 0.02267 * t12335 - 0.0019591358024691357 * t12337 + 0.04534 * t12341 + 0.06801 * t12345 - 0.02518888888888889 * t12348 - 0.04534 * t12351;
    (t12335, t12337, t12339, t12341, t12343, t12345, t12348, t12351, t12353)
}
