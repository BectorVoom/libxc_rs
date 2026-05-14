//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 594/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk594<F: Float>(t4980: F, t4996: F, t4999: F, t5013: F, t1519: F, t327: F, t5308: F, t5022: F, t1475: F, t1506: F, t1214: F, t1610: F, t93: F, t5039: F, t5045: F, t5068: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t5703 = 0.14975624337724558 * t4980;
    let t5706 = 0.1110086767380779 * t4996;
    let t5707 = 0.09983749558483038 * t4999;
    let t5710 = 0.29951248675449116 * t5013;
    let t5711 = t327 * t1519;
    let t5712 = t5711 * t5308;
    let t5714 = 0.020557162358903314 * t5022;
    let t5716 = t1506 * t1475;
    let t5717 = t1610 * t1214;
    let t5718 = t93 * t5717;
    let t5731 = 11.879313099038017 * t5039;
    let t5733 = 7.919542066025344 * t5045;
    let t5739 = 2.6398473553417814 * t5068;
    (t5703, t5706, t5707, t5710, t5711, t5712, t5714, t5716, t5718, t5731, t5733, t5739)
}
