//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 212/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk212<F: Float>(t767: F, t772: F, t131: F, t179: F, t192: F, t205: F, t571: F, t575: F, t704: F, t709: F, t713: F, t723: F, t727: F, t728: F, t739: F, t744: F, t750: F, t752: F, t754: F, t757: F, t98: F) -> (F, F) {
    let t773 = t767 * t772;
    let t776 = t571 + t575 - 2.2140749178833072 * t704 * t98 + 2.2140749178833072 * t192 * t709 - 18.635258017632964 * t179 * t713 - 18.635258017632964 * t179 * t709 + t723 + 2.2140749178833072 * t192 * t713 - 0.5923479790153209 * t727 * t131 * t728 + t739 + 2.3693919160612835 * t205 * t744 + t750 - t752 - t754 - 22.07984838129906 * t757 - 2.9824072957409817 * t773 * t98;
    (t773, t776)
}
