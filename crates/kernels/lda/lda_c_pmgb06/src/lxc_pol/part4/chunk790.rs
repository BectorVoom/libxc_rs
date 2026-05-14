//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 790/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk790<F: Float>(t5566: F, t5595: F, t5898: F, t5913: F, t2236: F, t73: F, t2432: F, t707: F, t23: F, t342: F, t2377: F, t3537: F, t1212: F, t2381: F, t4433: F, t4434: F) -> (F, F, F, F, F, F, F) {
    let t5915 = t5566 + t5595 + t5898 + t5913;
    let t5934 = t73 * t2236;
    let t5937 = t707 * t2432;
    let t5939 = t342 * t23;
    let t5953 = t3537 * t2377;
    let t5958 = t1212 * t2381;
    let t5961 = -t4433 - t4434;
    (t5915, t5934, t5937, t5939, t5953, t5958, t5961)
}
