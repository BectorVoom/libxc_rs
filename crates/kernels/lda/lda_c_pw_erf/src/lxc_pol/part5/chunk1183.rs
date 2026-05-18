//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1183/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1183<F: Float>(t3794: F, t7589: F, t1325: F, t1440: F, t15975: F, t806: F, t2098: F, t6979: F, t1472: F, t7558: F, t4804: F, t21489: F, t21494: F, t21496: F, t21498: F, t21500: F, t21505: F, t21509: F, t21513: F) -> (F, F, F, F, F, F) {
    let t21515 = F::new(4.0) / F::new(5.0) * t3794 * t7589;
    let t21519 = F::new(4.0) / F::new(5.0) * t1325 * t1440 * t15975 * t806;
    let t21523 = F::new(4.0) / F::new(5.0) * t1325 * t1440 * t6979 * t2098;
    let t21525 = F::new(4.0) / F::new(5.0) * t1472 * t7558;
    let t21527 = F::new(4.0) / F::new(5.0) * t4804 * t7589;
    let t21528 = -t21489 + t21494 + t21496 - t21498 - t21500 - t21505 + t21509 + t21513 - t21515 - t21519 - t21523 + t21525 - t21527;
    (t21515, t21519, t21523, t21525, t21527, t21528)
}
