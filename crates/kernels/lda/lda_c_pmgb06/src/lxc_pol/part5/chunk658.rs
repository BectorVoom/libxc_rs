//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 658/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk658<F: Float>(t2226: F, t947: F, t2236: F, t377: F, t1295: F, t783: F, t5790: F, t69: F, t5806: F, t109: F, t370: F, t2247: F, t2249: F) -> (F, F, F, F, F, F, F) {
    let t5813 = t2226 * t947;
    let t5831 = t2236 * t377;
    let t5834 = t783 * t1295;
    let t5852 = t69 * t5790;
    let t5855 = F::new(1.1495033333333333) * t69 * t5806;
    let t5858 = t109 * t370;
    let t5860 = t2247 * t5858 * t2249;
    (t5813, t5831, t5834, t5852, t5855, t5858, t5860)
}
