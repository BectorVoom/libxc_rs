//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 736/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk736<F: Float>(t8276: F, t8286: F, t8298: F, t8313: F, t2254: F, t3166: F, t633: F, t2246: F, t650: F, t896: F, t2258: F, t694: F, t903: F, t609: F, t3767: F, t623: F) -> (F, F, F, F, F, F, F) {
    let t8315 = t8276 + t8286 + t8298 + t8313;
    let t8318 = t3166 * t2254 * t633;
    let t8322 = t896 * t2246 * t650;
    let t8326 = t903 * t2258 * t694;
    let t8330 = t903 * t2258 * t609;
    let t8331 = t3767 * t8330;
    let t8334 = t3166 * t2258 * t623;
    (t8315, t8318, t8322, t8326, t8330, t8331, t8334)
}
