//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 623/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk623<F: Float>(t2085: F, t4913: F, t1832: F, t4641: F, t2094: F, t489: F, t161: F, t1636: F, t831: F, t4637: F, t819: F, t955: F) -> (F, F, F, F, F, F, F) {
    let t4914 = t4913 * t2085;
    let t4916 = t4641 * t1832;
    let t4948 = t489 * t2094;
    let t4950 = F::new(2.0) / F::new(45.0) * t161 * t4948;
    let t4970 = F::new(2.0) / F::new(45.0) * t831 * t1636;
    let t5002 = F::new(0.015996296296296297) * t4637;
    let t5003 = t955 * t819;
    (t4914, t4916, t4948, t4950, t4970, t5002, t5003)
}
