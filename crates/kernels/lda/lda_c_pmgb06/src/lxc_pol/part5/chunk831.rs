//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 831/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk831<F: Float>(t109: F, t1282: F, t2247: F, t2249: F, t370: F, t409: F, t11404: F, t69: F, t11392: F, t1773: F, t2262: F, t2266: F, t1183: F, t1798: F, t297: F, t301: F) -> (F, F, F, F, F, F, F) {
    let t11475 = t109 * t1282;
    let t11485 = t2247 * t409 * t370 * t2249;
    let t11519 = t69 * t11404;
    let t11521 = t69 * t11392;
    let t11567 = t1773 * t2262;
    let t11568 = 0.15965645347006147 * t11567;
    let t11569 = t1773 * t2266;
    let t11600 = t297 * t1798 * t1183 * t301;
    (t11475, t11485, t11519, t11521, t11568, t11569, t11600)
}
