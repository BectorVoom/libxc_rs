//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1251/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1251<F: Float>(t19: F, t644: F, t647: F, t7337: F, t18280: F, t18292: F, t12874: F, t2532: F, t4763: F, t6954: F, t14005: F, t20179: F, t22403: F, t22405: F, t22407: F, t22411: F, t22412: F, t22418: F, t247: F, t251: F, t256: F) -> (F, F, F, F, F) {
    let t22422 = t7337 * t19 * t644 * t647;
    let t22424 = F::new(4.0) / F::new(45.0) * t18280;
    let t22425 = F::new(16.0) / F::new(15.0) * t18292;
    let t22427 = F::new(8.0) / F::new(5.0) * t12874 * t2532;
    let t22429 = F::new(8.0) / F::new(5.0) * t4763 * t6954;
    let t22430 = -t22403 - t22405 - t22407 + t22411 + t14005 - t22412 + t20179 * t247 * t251 * t256 / F::new(3.0) + t22418 / F::new(3.0) + F::new(0.06077777777777778) * t22422 - t22424 + t22425 - t22427 - t22429;
    (t22424, t22425, t22427, t22429, t22430)
}
