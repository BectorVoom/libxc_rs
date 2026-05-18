//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1041/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1041<F: Float>(t11101: F, t545: F, t11262: F, t11264: F, t11271: F, t11274: F, t11278: F, t11283: F, t11287: F, t11290: F, t11292: F, t1805: F, t1842: F, t2032: F, t2744: F, t2903: F, t6242: F, t6672: F, t6677: F, t6686: F, t6692: F, t6702: F, t6711: F, t6714: F) -> F {
    let t11294 = t545 * t11101;
    let t11297 = -t6672 + F::new(0.013716887843283197) * t11262 - F::new(6.211752672544321) * t11264 + F::new(2.2140749178833072) * t6677 - F::new(18.635258017632964) * t6686 - F::new(4.937333717448355) * t6692 + F::new(0.04115066352984959) * t6702 + F::new(2.2140749178833072) * t11271 - t6711 + F::new(2.2140749178833072) * t11274 * t6242 + F::new(2.2140749178833072) * t11278 + F::new(2.2140749178833072) * t2903 * t2032 - F::new(7.108175748183851) * t11283 * t2744 - F::new(7.108175748183851) * t1842 * t11287 - t6714 - F::new(0.6268457032291772) * t11290 + F::new(6.496391258193384) * t11292 + F::new(2.427516195194328) * t11294 * t1805;
    t11297
}
