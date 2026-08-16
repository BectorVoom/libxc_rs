//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1283/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1283<F: Float>(t1991: F, t22764: F, t519: F, t1325: F, t494: F, t5250: F, t7635: F, t542: F, t9700: F, t14200: F, t22713: F, t14205: F, t22717: F) -> (F, F, F, F, F) {
    let t22967 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t519 * t1991 * t22764;
    let t22971 = F::cast_from(64.0_f64) / F::cast_from(81.0_f64) * t1325 * t5250 * t7635 * t494;
    let t22975 = F::cast_from(32.0_f64) / F::cast_from(81.0_f64) * t519 * t9700 * t7635 * t542;
    let t22978 = F::cast_from(352.0_f64) / F::cast_from(243.0_f64) * t519 * t14200 * t22713;
    let t22981 = F::cast_from(64.0_f64) / F::cast_from(27.0_f64) * t519 * t14205 * t22717;
    (t22967, t22971, t22975, t22978, t22981)
}
