//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 912/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk912<F: Float>(t1672: F, t2946: F, t1665: F, t2759: F, t1671: F, t2913: F, t1904: F, t1222: F, t2751: F) -> (F, F, F, F) {
    let t11238 = t2946 * t1672;
    let t11243 = t2759 * t1665;
    let t11245 = t1671 * t2913;
    let t11246 = t1904 * t11245;
    let t11248 = t1222 * t2751;
    (t11238, t11243, t11246, t11248)
}
