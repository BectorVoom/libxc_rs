//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 632/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk632<F: Float>(t498: F, t6601: F, t1672: F, t1979: F, t1975: F, t129: F, t132: F, t1906: F, t1904: F, t1671: F, t1920: F, t1949: F, t1948: F, t1927: F, t6488: F, t1901: F, t6477: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6603 = 0.8357942709722364 * t498 * t6601;
    let t6604 = t1979 * t1672;
    let t6606 = t1975 * t1672;
    let t6611 = t129 * t132;
    let t6612 = t6611 * t1906;
    let t6613 = t1904 * t6612;
    let t6615 = t1671 * t1920;
    let t6616 = t1904 * t6615;
    let t6620 = t1671 * t1949;
    let t6622 = 0.027433775686566395 * t1948 * t6620;
    let t6624 = 12.423505345088643 * t1927 * t6488;
    let t6625 = t1901 * t6477;
    (t6603, t6604, t6606, t6611, t6613, t6616, t6622, t6624, t6625)
}
