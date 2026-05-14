//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 876/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk876<F: Float>(t2531: F, t3899: F, t1318: F, t2140: F, t2146: F, t2471: F, t3675: F, t542: F, t1440: F, t2098: F, t2186: F, t2166: F, t575: F, t6005: F, t574: F, t1325: F, t1446: F, t1472: F, t2153: F, t2171: F, t2178: F, t2540: F, t2544: F, t2550: F, t2558: F, t2562: F, t2566: F, t3794: F, t4804: F, t519: F, t5312: F, t5327: F, t571: F, t799: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t6894 = t3899 * t2531;
    let t6895 = t1318 * t6894;
    let t6897 = t2146 * t2140;
    let t6903 = t3675 * t2471;
    let t6904 = t6903 * t542;
    let t6905 = t1440 * t6904;
    let t6908 = t2186 * t2098;
    let t6909 = t1440 * t6908;
    let t6916 = t2166 * t2098;
    let t6917 = t1440 * t6916;
    let t6924 = t575 * t6005;
    let t6925 = t574 * t6924;
    let t6936 = t5312 - 16.0 / 45.0 * t6895 + 16.0 / 135.0 * t6897 - 8.0 / 45.0 * t1472 * t2562 - 8.0 / 45.0 * t1446 * t2566 - 4.0 / 5.0 * t519 * t6905 + 8.0 / 15.0 * t519 * t6909 - 8.0 / 15.0 * t4804 * t2558 - 8.0 / 15.0 * t3794 * t2558 - 8.0 / 15.0 * t1325 * t6917 - 16.0 / 45.0 * t2146 * t2153 + 4.0 / 45.0 * t1472 * t2540 + 4.0 / 45.0 * t571 * t6925 + 4.0 / 27.0 * t1472 * t2544 + 8.0 / 45.0 * t5327 * t799 + 16.0 / 45.0 * t2171 * t2178 + 4.0 / 45.0 * t1446 * t2550;
    (t6894, t6903, t6904, t6905, t6908, t6909, t6916, t6917, t6924, t6925, t6936)
}
