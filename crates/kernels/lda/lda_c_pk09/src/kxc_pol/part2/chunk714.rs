//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 714/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk714<F: Float>(t788: F, t7946: F, t89: F, t2152: F, t3213: F, t131: F, t707: F, t7831: F, t2143: F, t755: F, t409: F) -> (F, F, F, F) {
    let t7947 = t7946 * t788;
    let t7948 = t7947 * t89;
    let t7951 = t3213 * t2152;
    let t7952 = t131 * t7951;
    let t7955 = t707 * t7831;
    let t7956 = t131 * t7955;
    let t7961 = t755 * t2143;
    let t7962 = t409 * t7961;
    (t7948, t7952, t7956, t7962)
}
