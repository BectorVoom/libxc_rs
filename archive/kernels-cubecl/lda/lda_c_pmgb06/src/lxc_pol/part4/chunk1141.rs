//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1141/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1141<F: Float>(t1065: F, t2395: F, t248: F, t11090: F, t11092: F, t11095: F, t11097: F, t11099: F, t11101: F, t8640: F, t8644: F, t8647: F, t8651: F, t8655: F, t8659: F, t8663: F, t8668: F, t8675: F, t8684: F, t8685: F) -> F {
    let t14984 = t248 * t2395 * t1065;
    let t14993 = t8640 + t8644 - t8647 - t8651 + t8655 + t8659 + F::cast_from(0.00024415263074675396_f64) * t8663 + t8668 + t14984 + F::cast_from(120.0_f64) * t11090 + F::cast_from(80.0_f64) * t11092 - F::cast_from(48.0_f64) * t11095 + F::cast_from(96.0_f64) * t11097 + F::cast_from(160.0_f64) * t11099 - F::cast_from(240.0_f64) * t11101 + F::cast_from(1.1696447245269292_f64) * t8675 - t8684 - F::cast_from(2050.8037716432814_f64) * t8685;
    t14993
}
