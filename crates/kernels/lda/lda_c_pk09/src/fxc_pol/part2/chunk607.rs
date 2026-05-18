//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 607/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk607<F: Float>(t130: F, t4993: F, t93: F, t4992: F, t1222: F, t1286: F) -> (F, F, F, F) {
    let t4994 = t130 * t4993;
    let t4995 = t93 * t4994;
    let t4996 = t4992 * t4995;
    let t4997 = F::new(16.20073542583857) * t4996;
    let t4998 = t1222 * t1286;
    (t4995, t4996, t4997, t4998)
}
