//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1019/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1019<F: Float>(t519: F, t6347: F, t9723: F, t1318: F, t3854: F, t6255: F, t3802: F, t6326: F, t3859: F, t6331: F, t511: F, t7016: F) -> (F, F, F, F, F) {
    let t16935 = t519 * t9723 * t6347;
    let t16949 = t1318 * t3854 * t6255;
    let t16952 = t519 * t3802 * t6326;
    let t16955 = t519 * t3859 * t6331;
    let t16957 = t511 * t7016;
    (t16935, t16949, t16952, t16955, t16957)
}
