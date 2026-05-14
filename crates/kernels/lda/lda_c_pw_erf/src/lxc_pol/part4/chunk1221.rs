//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1221/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1221<F: Float>(t13883: F, t13885: F, t13906: F, t12299: F, t2007: F, t15694: F, t2171: F, t4750: F, t1315: F, t6988: F, t1997: F, t5327: F, t13915: F, t13917: F, t13919: F, t18046: F, t18048: F, t18050: F, t18052: F, t18111: F) -> (F, F, F, F, F, F, F, F, F) {
    let t18112 = 8.0 / 45.0 * t13883;
    let t18113 = 16.0 / 45.0 * t13885;
    let t18114 = 32.0 / 45.0 * t13906;
    let t18119 = 32.0 / 45.0 * t12299 * t2007;
    let t18121 = 32.0 / 45.0 * t15694 * t2007;
    let t18123 = 16.0 / 45.0 * t2171 * t4750;
    let t18125 = 16.0 / 45.0 * t6988 * t1315;
    let t18127 = 16.0 / 45.0 * t5327 * t1997;
    let t18128 = t18046 - t18048 - t18050 + t18052 + t18111 - t18112 - t18113 - t18114 + 0.2885611029963958 * t13915 + 0.4328416544945937 * t13917 - 0.19237406866426388 * t13919 + t18119 + t18121 - t18123 - t18125 - t18127;
    (t18112, t18113, t18114, t18119, t18121, t18123, t18125, t18127, t18128)
}
