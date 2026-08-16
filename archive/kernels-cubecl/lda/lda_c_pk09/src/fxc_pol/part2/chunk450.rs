//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 450/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk450<F: Float>(t2385: F, t2405: F, t1125: F, t134: F, t143: F, t161: F, t164: F, t179: F, t192: F, t200: F, t2154: F, t2305: F, t2314: F, t2319: F, t80: F, t812: F, t833: F, t843: F, t870: F, t872: F, t882: F, t884: F) -> (F, F) {
    let t2406 = t2385 + t2405;
    let t2408 = -F::cast_from(3.7610742193750633_f64) * t143 * t2154 - F::cast_from(0.6268457032291772_f64) * t2305 * t134 - t812 - t833 + t843 - t1125 * t2314 + F::cast_from(4.937333717448355_f64) * t161 * t2154 - F::cast_from(0.04115066352984959_f64) * t164 * t2319 + F::cast_from(18.635258017632964_f64) * t179 * t2154 - F::cast_from(2.2140749178833072_f64) * t192 * t2154 - F::cast_from(2.427516195194328_f64) * t200 * t2154 + t80 * t2406 - t870 + t872 + t882 + t884;
    (t2406, t2408)
}
