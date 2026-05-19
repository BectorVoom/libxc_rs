//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 882/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk882<F: Float>(t1200: F, t1329: F, t199: F, t3982: F, t1139: F, t566: F, t107: F, t2786: F, t701: F, t290: F, t8170: F, t3076: F, t432: F) -> (F, F, F, F, F, F) {
    let t9052 = t1329 * t1200;
    let t9061 = t3982 * t199;
    let t9063 = t1139 * t566;
    let t9066 = t107 * t2786 * t701;
    let t9070 = F::cast_from(19.1926369973667_f64) * t107 * t8170 * t290;
    let t9089 = t432 * t3076;
    (t9052, t9061, t9063, t9066, t9070, t9089)
}
