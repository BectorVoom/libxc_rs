//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 956/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk956<F: Float>(t301: F, t413: F, t5988: F, t5980: F, t76: F, t123: F, t317: F, t6104: F, t740: F, t73: F, t1122: F, t2395: F, t30: F) -> (F, F, F, F, F) {
    let t14789 = t5988 * t413 * t301;
    let t14797 = t76 * t5980;
    let t14852 = t123 * t740 * t6104 * t317;
    let t14875 = t73 * t5980;
    let t14939 = t2395 * t30 * t1122;
    (t14789, t14797, t14852, t14875, t14939)
}
