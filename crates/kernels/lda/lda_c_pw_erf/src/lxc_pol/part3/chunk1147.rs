//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1147/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1147<F: Float>(t13426: F, t1472: F, t4791: F, t4795: F, t4906: F, t529: F, t4849: F, t519: F, t12695: F, t4633: F, t1124: F, t1458: F, t197: F) -> (F, F, F, F, F, F) {
    let t13427 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t13426;
    let t13428 = t1472 * t4791;
    let t13429 = F::cast_from(32.0_f64) / F::cast_from(45.0_f64) * t13428;
    let t13430 = t1472 * t4795;
    let t13431 = F::cast_from(16.0_f64) / F::cast_from(27.0_f64) * t13430;
    let t13432 = t4906 * t529;
    let t13434 = t519 * t13432 * t4849;
    let t13435 = F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t13434;
    let t13437 = t519 * t12695 * t4633;
    let t13438 = F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t13437;
    let t13440 = t1124 * t1458 * t197;
    (t13427, t13429, t13431, t13435, t13438, t13440)
}
