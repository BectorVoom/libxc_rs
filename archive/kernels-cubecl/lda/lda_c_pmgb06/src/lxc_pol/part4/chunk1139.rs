//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1139/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1139<F: Float>(t11067: F, t11070: F, t8545: F, t8548: F, t8552: F, t8553: F, t8555: F, t8559: F, t8560: F, t8564: F, t8567: F, t8570: F, t8572: F, t8576: F, t8580: F, t8583: F, t8586: F, t8589: F) -> F {
    let t14966 = -F::cast_from(8.0_f64) * t11067 - F::cast_from(8.0_f64) * t11070 - F::cast_from(24.0_f64) * t8545 + F::cast_from(2.0_f64) * t8548 - t8552 + F::cast_from(0.03253074390090522_f64) * t8553 - F::cast_from(0.06506148780181044_f64) * t8555 - t8559 - F::cast_from(0.04337432520120696_f64) * t8560 - t8564 - t8567 + t8570 + F::cast_from(0.01084358130030174_f64) * t8572 + F::cast_from(0.9631946627535314_f64) * t8576 + t8580 + t8583 + t8586 + t8589;
    t14966
}
