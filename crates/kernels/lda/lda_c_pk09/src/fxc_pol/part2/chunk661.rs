//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 661/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk661<F: Float>(t3262: F, t7608: F, t2210: F, t2214: F, t2975: F, t2981: F, t3265: F, t3268: F, t3826: F, t3829: F, t7578: F, t7584: F, t7586: F, t7590: F, t7598: F, t7602: F) -> (F,) {
    let t7609 = t3262 * t7608;
    let t7611 = 4.937333717448355 * t2975 - 4.937333717448355 * t2981 + 3.7610742193750633 * t3265 * t7578 - 1.8805371096875316 * t3268 * t2214 - 1.8805371096875316 * t7584 * t7586 + 19.489173774580152 * t3826 * t7590 + 38.978347549160304 * t3826 * t7578 - 19.489173774580152 * t3829 * t2214 + 3.7610742193750633 * t3265 * t7598 + 1.8805371096875316 * t3265 * t7602 - 1.8805371096875316 * t3268 * t2210 + 1.8805371096875316 * t7609;
    (t7611,)
}
