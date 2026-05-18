//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 916/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk916<F: Float>(t34: F, t3615: F, t109: F, t1282: F, t2247: F, t2249: F, t370: F, t409: F, t11404: F, t69: F, t11392: F, t1773: F, t2262: F) -> (F, F, F, F, F, F) {
    let t11470 = t34 * t3615;
    let t11475 = t109 * t1282;
    let t11485 = t2247 * t409 * t370 * t2249;
    let t11519 = t69 * t11404;
    let t11521 = t69 * t11392;
    let t11567 = t1773 * t2262;
    (t11470, t11475, t11485, t11519, t11521, t11567)
}
