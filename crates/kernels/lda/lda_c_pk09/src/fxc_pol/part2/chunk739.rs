//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 739/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk739<F: Float>(t8360: F, t8373: F, t862: F, t89: F, t161: F, t3847: F, t3855: F, t3906: F, t3908: F, t3983: F, t3984: F, t3986: F, t3993: F, t4001: F, t4005: F, t7706: F, t7776: F, t98: F) -> (F, F) {
    let t8374 = t8360 + t8373;
    let t8375 = t8374 * t862;
    let t8376 = t8375 * t89;
    let t8385 = -t3847 - t3855 - 4.738783832122567 * t3906 - 3.7610742193750633 * t3908 + 4.937333717448355 * t8376 * t98 - 4.937333717448355 * t161 * t7776 - 4.937333717448355 * t161 * t7706 + t3983 - 1.4760499452555382 * t3984 - 1.4760499452555382 * t3986 + t3993 - t4001 - t4005;
    (t8374, t8385)
}
