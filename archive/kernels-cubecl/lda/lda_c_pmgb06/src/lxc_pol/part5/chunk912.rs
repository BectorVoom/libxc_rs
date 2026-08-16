//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 912/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk912<F: Float>(t11227: F, t8276: F, t3615: F, t63: F, t370: F, t38: F, t8281: F, t342: F, t569: F, t99: F, t1271: F, t2229: F) -> (F, F, F, F, F, F) {
    let t11228 = t8276 * t11227;
    let t11230 = t63 * t3615;
    let t11234 = t38 * t370;
    let t11237 = t8281 * t11227;
    let t11303 = t99 * t569 * t342;
    let t11304 = t1271 * t2229 * t11303;
    (t11228, t11230, t11234, t11237, t11303, t11304)
}
