//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 394/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk394<F: Float>(t1826: F, t1924: F, t1978: F, t2129: F, t417: F, t1130: F, t1135: F, t1658: F, t1663: F, t564: F, t560: F, t561: F, t208: F, t42: F) -> (F, F, F, F, F) {
    let t2131 = t1826 + t1924 + t1978 + t2129;
    let t2132 = t417 * t2131;
    let t2134 = t564 / 4.0 + t1130 / 4.0 + t1135 / 8.0 + t1658 / 8.0 + t1663 / 8.0 + t2132 / 8.0;
    let t2137 = 2.0 * t560 + 2.0 * t561;
    let t2138 = t2137 * t208;
    let t2140 = -t42 - t560;
    (t2131, t2134, t2137, t2138, t2140)
}
