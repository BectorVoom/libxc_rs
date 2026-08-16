//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 601/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk601<F: Float>(t1156: F, t266: F, t1161: F, t1179: F, t1185: F, t258: F, t4787: F, t4789: F, t4793: F, t272: F, t1171: F, t256: F) -> (F, F, F, F, F) {
    let t4821 = t1156 * t266;
    let t4822 = t1179 * t1161;
    let t4823 = t4822 * t1185;
    let t4824 = t4821 * t4823;
    let t4830 = F::cast_from(1.8073681049360268_f64) * t4787 + F::cast_from(15.112064760386344_f64) * t4789 - F::cast_from(12.010155044502033_f64) * t258 + F::cast_from(0.5833333333333334_f64) * t4793;
    let t4831 = t272 * t4830;
    let t4833 = F::cast_from(1.28_f64) * t1156 * t4831;
    let t4837 = t256 * t1171;
    (t4821, t4824, t4830, t4833, t4837)
}
