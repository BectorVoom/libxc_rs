//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 756/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk756<F: Float>(t1440: F, t6916: F, t575: F, t6005: F, t574: F, t1325: F, t1446: F, t1472: F, t2146: F, t2153: F, t2171: F, t2178: F, t2540: F, t2544: F, t2550: F, t2558: F, t2562: F, t2566: F, t3794: F, t4804: F, t519: F, t5312: F, t5327: F, t571: F, t6895: F, t6897: F, t6905: F, t6909: F, t799: F) -> (F, F, F, F) {
    let t6917 = t1440 * t6916;
    let t6924 = t575 * t6005;
    let t6925 = t574 * t6924;
    let t6936 = t5312 - F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t6895 + F::cast_from(16.0_f64) / F::cast_from(135.0_f64) * t6897 - F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t1472 * t2562 - F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t1446 * t2566 - F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t519 * t6905 + F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t519 * t6909 - F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t4804 * t2558 - F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t3794 * t2558 - F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t1325 * t6917 - F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t2146 * t2153 + F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t1472 * t2540 + F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t571 * t6925 + F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t1472 * t2544 + F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t5327 * t799 + F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t2171 * t2178 + F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t1446 * t2550;
    (t6917, t6924, t6925, t6936)
}
