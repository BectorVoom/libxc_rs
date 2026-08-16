//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 450/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk450(t2385: f64, t2405: f64, t1125: f64, t134: f64, t143: f64, t161: f64, t164: f64, t179: f64, t192: f64, t200: f64, t2154: f64, t2305: f64, t2314: f64, t2319: f64, t80: f64, t812: f64, t833: f64, t843: f64, t870: f64, t872: f64, t882: f64, t884: f64) -> (f64, f64) {
    let t2406 = t2385 + t2405;
    let t2408 = -3.7610742193750633_f64 * t143 * t2154 - 0.6268457032291772_f64 * t2305 * t134 - t812 - t833 + t843 - t1125 * t2314 + 4.937333717448355_f64 * t161 * t2154 - 0.04115066352984959_f64 * t164 * t2319 + 18.635258017632964_f64 * t179 * t2154 - 2.2140749178833072_f64 * t192 * t2154 - 2.427516195194328_f64 * t200 * t2154 + t80 * t2406 - t870 + t872 + t882 + t884;
    (t2406, t2408)
}
