//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 978/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk978<F: Float>(t5170: F, t565: F, t1498: F, t2123: F, t3416: F, t4785: F, t2010: F, t571: F, t9313: F, t10654: F, t1949: F, t3863: F, t4837: F, t1446: F, t4750: F, t1472: F, t5286: F) -> (F, F, F, F, F, F, F, F) {
    let t13041 = t565 * t5170;
    let t13042 = 8.0 / 15.0 * t13041;
    let t13043 = t1498 * t2123;
    let t13044 = 4.0 / 15.0 * t13043;
    let t13046 = 32.0 / 15.0 * t3416 * t4785;
    let t13048 = t571 * t9313 * t2010;
    let t13049 = 8.0 / 135.0 * t13048;
    let t13051 = t571 * t10654 * t1949;
    let t13052 = 16.0 / 135.0 * t13051;
    let t13054 = t571 * t3863 * t4837;
    let t13055 = 8.0 / 45.0 * t13054;
    let t13057 = 8.0 / 15.0 * t1446 * t4750;
    let t13059 = 8.0 / 15.0 * t1472 * t5286;
    (t13042, t13044, t13046, t13049, t13052, t13055, t13057, t13059)
}
