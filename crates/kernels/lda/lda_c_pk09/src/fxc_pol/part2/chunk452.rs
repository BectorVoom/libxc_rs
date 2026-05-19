//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 452/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk452<F: Float>(t2354: F, t89: F, t143: F, t151: F, t155: F, t179: F, t2210: F, t2214: F, t2419: F, t886: F, t888: F, t921: F, t925: F, t946: F, t959: F, t98: F, t982: F, t986: F) -> (F, F) {
    let t2426 = t2354 * t89;
    let t2437 = -t886 + t888 + t921 + t925 - F::cast_from(1.8805371096875316_f64) * t151 * t2214 + F::cast_from(1.8805371096875316_f64) * t2419 * t98 - F::cast_from(19.489173774580152_f64) * t155 * t2210 - F::cast_from(19.489173774580152_f64) * t155 * t2214 + F::cast_from(19.489173774580152_f64) * t2426 * t98 + F::cast_from(3.7610742193750633_f64) * t143 * t2210 + F::cast_from(3.7610742193750633_f64) * t143 * t2214 - F::cast_from(18.635258017632964_f64) * t179 * t2210 - F::cast_from(18.635258017632964_f64) * t179 * t2214 - t946 + t959 - t982 + t986;
    (t2426, t2437)
}
