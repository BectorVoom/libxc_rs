//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 638/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk638<F: Float>(t481: F, t6805: F, t6700: F, t68: F, t1800: F, t142: F, t1991: F, t902: F, t92: F, t6525: F, t1947: F, t1905: F, t1948: F, t6586: F, t551: F, t6517: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6806 = t481 * t6805;
    let t6810 = t6700 * t68;
    let t6811 = t6810 * t1800;
    let t6813 = t1991 * t142;
    let t6814 = t92 * t902;
    let t6816 = t6813 * t6814 * t6525;
    let t6818 = t1947 * t142;
    let t6822 = t1905 * t6525;
    let t6823 = t1948 * t6822;
    let t6825 = t6586 * t142;
    let t6827 = t6825 * t551 * t6517;
    (t6806, t6811, t6813, t6814, t6816, t6818, t6823, t6825, t6827)
}
