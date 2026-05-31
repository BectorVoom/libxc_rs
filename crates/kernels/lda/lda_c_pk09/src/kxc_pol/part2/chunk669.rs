//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 669/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk669<F: Float>(t454: F, t6223: F, t1948: F, t1927: F, t6196: F, t1895: F, t1893: F, t529: F, t532: F, t1892: F, t1792: F, t1884: F) -> (F, F, F, F, F) {
    let t6224 = t454 * t6223;
    let t6225 = t1948 * t6224;
    let t6227 = t1927 * t6196;
    let t6229 = t1895 * t1895;
    let t6230 = F::cast_from(1.0_f64) / t6229;
    let t6233 = t529 * t1893;
    let t6236 = t532 * t532;
    let t6237 = F::cast_from(1.0_f64) / t6236;
    let t6238 = t1892 * t6237;
    let t6240 = t1792 * t6238 - F::cast_from(2.0_f64) * t1884 * t6233;
    (t6225, t6227, t6230, t6236, t6240)
}
