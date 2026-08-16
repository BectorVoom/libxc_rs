//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 634/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk634<F: Float>(t5293: F, t93: F, t1470: F, t1403: F, t4979: F, t1240: F, t1364: F, t310: F, t1337: F, t1471: F, t623: F, t333: F) -> (F, F, F, F, F) {
    let t5294 = t93 * t5293;
    let t5296 = F::cast_from(7.108175748183851_f64) * t1470 * t5294;
    let t5298 = F::cast_from(2.2140749178833072_f64) * t1403 * t4979;
    let t5303 = t1364 * t1240;
    let t5304 = t310 * t5303;
    let t5305 = t1337 * t5304;
    let t5307 = t1471 * t623;
    let t5308 = t333 * t5307;
    (t5294, t5296, t5298, t5305, t5308)
}
