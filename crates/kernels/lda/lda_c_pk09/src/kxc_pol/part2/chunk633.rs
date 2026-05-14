//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 633/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk633<F: Float>(t490: F, t6601: F, t508: F, t6501: F, t6505: F, t6508: F, t6519: F, t6522: F, t6527: F, t6319: F, t6325: F, t6547: F, t6550: F, t6464: F, t1842: F, t6593: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t6628 = 1.6715885419444727 * t490 * t6601;
    let t6630 = 2.1943705410881575 * t508 * t6601;
    let t6633 = 2.0 * t6501;
    let t6634 = 2.0 * t6505;
    let t6635 = 2.6666666666666665 * t6508;
    let t6637 = 8.0 * t6519;
    let t6638 = 2.6666666666666665 * t6522;
    let t6639 = 8.0 * t6527;
    let t6642 = 0.505765839233979 * t6319;
    let t6649 = 0.337177226155986 * t6325;
    let t6650 = 0.2222222222222222 * t6547;
    let t6651 = 2.6666666666666665 * t6550;
    let t6655 = 0.112392408718662 * t6464;
    let t6662 = t1842 * t6593;
    (t6628, t6630, t6633, t6634, t6635, t6637, t6638, t6639, t6642, t6649, t6650, t6651, t6655, t6662)
}
