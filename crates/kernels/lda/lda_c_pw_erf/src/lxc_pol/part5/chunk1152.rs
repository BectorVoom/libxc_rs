//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1152/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1152<F: Float>(t3402: F, t519: F, t542: F, t7639: F, t1446: F, t7692: F, t1313: F, t6557: F, t806: F, t2098: F, t2437: F, t7695: F) -> (F, F, F, F, F) {
    let t21173 = F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t519 * t3402 * t7639 * t542;
    let t21175 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t1446 * t7692;
    let t21179 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t519 * t1313 * t6557 * t806;
    let t21183 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t519 * t1313 * t2437 * t2098;
    let t21185 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t1446 * t7695;
    (t21173, t21175, t21179, t21183, t21185)
}
