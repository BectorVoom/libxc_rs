//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 896/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk896<F: Float>(t9508: F, t2920: F, t350: F, t1413: F, t1463: F, t1486: F, t947: F, t1478: F, t2940: F, t2914: F, t1830: F, t508: F) -> (F, F, F, F, F, F, F, F) {
    let t9509 = F::cast_from(1.0_f64) / t9508;
    let t9522 = t350 * t2920;
    let t9525 = F::cast_from(1.0_f64) / t1463 / t1413;
    let t9530 = t947 * t1486;
    let t9532 = t947 * t1478;
    let t9534 = t350 * t2940;
    let t9537 = t350 * t2914;
    let t9552 = t1830 * t508;
    (t9509, t9522, t9525, t9530, t9532, t9534, t9537, t9552)
}
