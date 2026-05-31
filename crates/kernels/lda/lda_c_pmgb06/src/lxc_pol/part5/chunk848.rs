//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 848/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk848<F: Float>(t1100: F, t79: F, t4320: F, t711: F, t715: F, t20: F, t369: F, t3501: F, t3502: F, t642: F, t3509: F, t3510: F) -> (F, F, F, F, F, F, F) {
    let t8193 = t79 * t1100;
    let t8194 = F::cast_from(120.0_f64) * t8193;
    let t8208 = t4320 * t711;
    let t8211 = F::cast_from(0.7805426614091894_f64) * t4320 * t715;
    let t8245 = F::cast_from(1.0_f64) / t369 / t20;
    let t8263 = F::cast_from(15.589466666666667_f64) * t3501 * t3502 * t642;
    let t8266 = F::cast_from(2.6116266666666665_f64) * t3509 * t3510 * t642;
    (t8193, t8194, t8208, t8211, t8245, t8263, t8266)
}
