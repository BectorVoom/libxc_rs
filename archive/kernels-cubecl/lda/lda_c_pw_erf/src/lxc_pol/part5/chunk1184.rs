//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1184/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1184<F: Float>(t2188: F, t6988: F, t1390: F, t7792: F, t1440: F, t519: F, t542: F, t529: F, t1325: F, t494: F, t2171: F, t7004: F) -> (F, F, F, F) {
    let t21530 = F::cast_from(8.0_f64) / F::cast_from(5.0_f64) * t6988 * t2188;
    let t21531 = t1390 * t7792;
    let t21535 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t519 * t1440 * t21531 * t542;
    let t21536 = t529 * t7792;
    let t21540 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t1325 * t1440 * t21536 * t494;
    let t21542 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t2171 * t7004;
    (t21530, t21535, t21540, t21542)
}
