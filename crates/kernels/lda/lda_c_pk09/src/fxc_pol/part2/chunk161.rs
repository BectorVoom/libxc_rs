//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 161/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk161<F: Float>(t514: F, t95: F, t333: F, t441: F, t305: F) -> (F, F, F, F, F) {
    let t515 = t95 * t514;
    let t516 = t333 * t515;
    let t520 = F::cast_from(1.5625_f64) * t441 + F::cast_from(0.3208669506079574_f64);
    let t523 = F::atan(F::cast_from(0.16004110557090126_f64) / t520);
    let t524 = t523 * t305;
    (t515, t516, t520, t523, t524)
}
