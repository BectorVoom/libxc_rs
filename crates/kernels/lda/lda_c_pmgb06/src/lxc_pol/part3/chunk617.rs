//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 617/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk617<F: Float>(t1282: F, t342: F, t103: F, t37: F, t28: F, t39: F, t247: F, t1227: F, t361: F, t38: F, t61: F, t939: F) -> (F, F, F, F, F, F, F) {
    let t3494 = t1282 * t342;
    let t3500 = F::new(1.0) / t37 / t103 / F::new(4.0);
    let t3501 = param_hyb_omega_0 * t3500;
    let t3502 = t39 * t28;
    let t3505 = F::cast_from(1.9486833333333333_f64) * t3501 * t3502 * t247;
    let t3508 = F::new(17.53815) * t38 * t361 * t1227;
    let t3509 = t61 * t939;
    (t3494, t3500, t3501, t3502, t3505, t3508, t3509)
}
