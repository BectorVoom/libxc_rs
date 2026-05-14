//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1009/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1009<F: Float>(t1313: F, t519: F, t6557: F, t806: F, t2098: F, t2437: F, t1446: F, t7695: F, t15852: F, t739: F, t1326: F, t34: F, t6330: F, t4829: F, t7698: F, t15867: F) -> (F, F, F, F, F, F, F, F, F) {
    let t21179 = 4.0 / 15.0 * t519 * t1313 * t6557 * t806;
    let t21183 = 4.0 / 15.0 * t519 * t1313 * t2437 * t2098;
    let t21185 = 8.0 / 15.0 * t1446 * t7695;
    let t21186 = t15852 * t739;
    let t21189 = 8.0 / 15.0 * t519 * t1326 * t21186;
    let t21190 = t6330 * t34;
    let t21193 = 16.0 / 15.0 * t519 * t4829 * t21190;
    let t21195 = 4.0 / 9.0 * t1446 * t7698;
    let t21196 = t15867 * t739;
    (t21179, t21183, t21185, t21186, t21189, t21190, t21193, t21195, t21196)
}
