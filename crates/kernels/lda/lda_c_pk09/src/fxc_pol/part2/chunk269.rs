//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 269/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk269<F: Float>(t1243: F, t280: F, t281: F, t226: F, t68: F) -> (F, F, F, F) {
    let t1244 = 3.2084841915276807 * t1243;
    let t1246 = 1.0 / t281 / t280;
    let t1247 = t226 * t1246;
    let t1248 = t1247 * t68;
    (t1244, t1246, t1247, t1248)
}
