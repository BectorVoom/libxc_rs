//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 965/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk965<F: Float>(t1313: F, t2954: F, t519: F, t5220: F, t5295: F, t9304: F, t3677: F, t789: F, t9351: F, t10467: F, t1996: F, t3802: F, t5425: F, t3430: F, t4763: F, t1315: F, t5327: F) -> (F, F, F, F, F, F, F) {
    let t12829 = 8.0 / 15.0 * t519 * t1313 * t5220 * t2954;
    let t12831 = t519 * t9304 * t5295;
    let t12832 = 16.0 / 45.0 * t12831;
    let t12836 = 8.0 / 15.0 * t519 * t9351 * t789 * t3677;
    let t12838 = t519 * t10467 * t1996;
    let t12839 = 8.0 / 135.0 * t12838;
    let t12841 = t519 * t3802 * t5425;
    let t12842 = 16.0 / 45.0 * t12841;
    let t12844 = 8.0 / 9.0 * t4763 * t3430;
    let t12846 = 8.0 / 15.0 * t5327 * t1315;
    (t12829, t12832, t12836, t12839, t12842, t12844, t12846)
}
