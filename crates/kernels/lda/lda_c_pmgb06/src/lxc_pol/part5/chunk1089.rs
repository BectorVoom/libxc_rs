//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1089/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1089<F: Float>(t460: F, t7465: F, t6705: F, t815: F, t1874: F, t2592: F, t16442: F, t16444: F, t16448: F, t16455: F, t6626: F, t802: F) -> (F, F, F, F, F, F, F, F) {
    let t20107 = t7465 * t460 / F::new(30.0);
    let t20109 = t6705 * t815 / F::new(10.0);
    let t20111 = t2592 * t1874 / F::new(10.0);
    let t20112 = t16442 / F::new(15.0);
    let t20113 = t16444 / F::new(15.0);
    let t20115 = F::new(2.0) / F::new(15.0) * t16448;
    let t20116 = F::new(2.0) / F::new(15.0) * t16455;
    let t20120 = t802 * t6626;
    (t20107, t20109, t20111, t20112, t20113, t20115, t20116, t20120)
}
