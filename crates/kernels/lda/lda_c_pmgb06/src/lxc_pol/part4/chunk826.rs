//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 826/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk826<F: Float>(t856: F, t97: F, t1377: F, t2342: F, t27: F, t545: F, t2345: F, t1366: F, t2349: F, t3019: F, t3026: F, t3028: F, t5089: F, t5093: F, t5097: F, t5101: F, t5104: F, t5107: F, t5112: F, t5114: F) -> (F, F, F, F, F, F, F, F) {
    let t5649 = t856 * t97;
    let t5650 = t5649 * t1377;
    let t5652 = t2342 * t27;
    let t5654 = F::cast_from(0.21642082724729686_f64) * t5652 * t545;
    let t5655 = t2345 * t27;
    let t5656 = t5655 * t545;
    let t5658 = t2349 * t1366;
    let t5660 = -t5089 + t5093 + t5097 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t3019 + t3026 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t3028 + t5101 - t5104 - t5107 + t5112 - t5114 + F::cast_from(0.011181742741110338_f64) * t5650 + t5654 + F::cast_from(0.21642082724729686_f64) * t5656 + F::cast_from(0.07214027574909895_f64) * t5658;
    (t5649, t5650, t5652, t5654, t5655, t5656, t5658, t5660)
}
