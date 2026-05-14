//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1031/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1031<F: Float>(t1466: F, t15619: F, t571: F, t833: F, t1440: F, t2098: F, t519: F, t7002: F, t3794: F, t7589: F, t1325: F, t15975: F, t806: F, t6979: F, t1472: F, t7558: F) -> (F, F, F, F, F, F) {
    let t21509 = 4.0 / 5.0 * t571 * t1466 * t15619 * t833;
    let t21513 = 4.0 / 5.0 * t519 * t1440 * t7002 * t2098;
    let t21515 = 4.0 / 5.0 * t3794 * t7589;
    let t21519 = 4.0 / 5.0 * t1325 * t1440 * t15975 * t806;
    let t21523 = 4.0 / 5.0 * t1325 * t1440 * t6979 * t2098;
    let t21525 = 4.0 / 5.0 * t1472 * t7558;
    (t21509, t21513, t21515, t21519, t21523, t21525)
}
