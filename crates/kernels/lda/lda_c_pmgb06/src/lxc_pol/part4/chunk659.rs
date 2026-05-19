//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 659/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk659<F: Float>(t3500: F, t28: F, t39: F, t247: F, t61: F, t939: F, t64: F, t1830: F, t366: F, t349: F, t1179: F, t54: F, t55: F, t56: F, param_hyb_omega_0: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3501 = param_hyb_omega_0 * t3500;
    let t3502 = t39 * t28;
    let t3505 = F::cast_from(1.9486833333333333_f64) * t3501 * t3502 * t247;
    let t3509 = t61 * t939;
    let t3510 = t64 * t28;
    let t3513 = F::cast_from(0.3264533333333333_f64) * t3509 * t3510 * t247;
    let t3515 = F::cast_from(0.7617244444444444_f64) * t366 * t1830;
    let t3517 = F::cast_from(1.5156425925925925_f64) * t349 * t1830;
    let t3521 = F::new(7.0) / F::new(27.0) * t54 * t55 * t1179 * t56;
    (t3501, t3502, t3505, t3509, t3510, t3513, t3515, t3517, t3521)
}
