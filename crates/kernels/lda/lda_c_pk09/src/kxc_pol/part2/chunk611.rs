//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 611/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk611<F: Float>(t1927: F, t6292: F, t1468: F, t496: F, t1747: F, t4993: F, t95: F, t333: F) -> (F, F, F) {
    let t6294 = 18.635258017632964 * t1927 * t6292;
    let t6299 = t496 * t1468;
    let t6300 = t6299 * t1747;
    let t6301 = t95 * t4993;
    let t6302 = t333 * t6301;
    (t6294, t6300, t6302)
}
