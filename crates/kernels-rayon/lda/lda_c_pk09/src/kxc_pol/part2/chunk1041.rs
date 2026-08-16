//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1041/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1041(t11101: f64, t545: f64, t11262: f64, t11264: f64, t11271: f64, t11274: f64, t11278: f64, t11283: f64, t11287: f64, t11290: f64, t11292: f64, t1805: f64, t1842: f64, t2032: f64, t2744: f64, t2903: f64, t6242: f64, t6672: f64, t6677: f64, t6686: f64, t6692: f64, t6702: f64, t6711: f64, t6714: f64) -> f64 {
    let t11294 = t545 * t11101;
    let t11297 = -t6672 + 0.013716887843283197_f64 * t11262 - 6.211752672544321_f64 * t11264 + 2.2140749178833072_f64 * t6677 - 18.635258017632964_f64 * t6686 - 4.937333717448355_f64 * t6692 + 0.04115066352984959_f64 * t6702 + 2.2140749178833072_f64 * t11271 - t6711 + 2.2140749178833072_f64 * t11274 * t6242 + 2.2140749178833072_f64 * t11278 + 2.2140749178833072_f64 * t2903 * t2032 - 7.108175748183851_f64 * t11283 * t2744 - 7.108175748183851_f64 * t1842 * t11287 - t6714 - 0.6268457032291772_f64 * t11290 + 6.496391258193384_f64 * t11292 + 2.427516195194328_f64 * t11294 * t1805;
    t11297
}
