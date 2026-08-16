//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1115/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1115<F: Float>(t5170: F, t565: F, t1498: F, t2123: F, t3416: F, t4785: F, t2010: F, t571: F, t9313: F, t10654: F, t1949: F, t3863: F, t4837: F) -> (F, F, F, F, F, F) {
    let t13041 = t565 * t5170;
    let t13042 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t13041;
    let t13043 = t1498 * t2123;
    let t13044 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t13043;
    let t13046 = F::cast_from(32.0_f64) / F::cast_from(15.0_f64) * t3416 * t4785;
    let t13048 = t571 * t9313 * t2010;
    let t13049 = F::cast_from(8.0_f64) / F::cast_from(135.0_f64) * t13048;
    let t13051 = t571 * t10654 * t1949;
    let t13052 = F::cast_from(16.0_f64) / F::cast_from(135.0_f64) * t13051;
    let t13054 = t571 * t3863 * t4837;
    (t13042, t13044, t13046, t13049, t13052, t13054)
}
