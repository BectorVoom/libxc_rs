//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 590/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk590<F: Float>(t1336: F, t1382: F, t1625: F, t1469: F, t1475: F, t1214: F, t1471: F, t93: F, t1240: F, t741: F, t623: F, t1470: F, t1386: F, t1468: F, t392: F, t1387: F) -> (F, F, F, F, F, F) {
    let t5594 = t1382 * t1336;
    let t5595 = t5594 * t1625;
    let t5603 = t1469 * t1475;
    let t5604 = t1471 * t1214;
    let t5605 = t93 * t5604;
    let t5606 = t5603 * t5605;
    let t5608 = t741 * t1240;
    let t5609 = t5608 * t623;
    let t5610 = t93 * t5609;
    let t5611 = t1470 * t5610;
    let t5613 = t1386 * t1475;
    let t5623 = t392 * t1468;
    let t5624 = t5623 * t1387;
    (t5595, t5603, t5606, t5611, t5613, t5624)
}
