//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 967/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk967<F: Float>(t1423: F, t6472: F, t5211: F, t6382: F, t436: F, t6705: F, t1517: F, t2592: F, t161: F, t489: F, t6231: F, t5499: F, t6536: F) -> (F, F, F, F, F, F) {
    let t15772 = t1423 * t6472;
    let t15774 = t5211 * t6382;
    let t15793 = t6705 * t436;
    let t15795 = t2592 * t1517;
    let t15807 = t161 * t489 * t6231;
    let t15829 = t5499 * t6536;
    (t15772, t15774, t15793, t15795, t15807, t15829)
}
