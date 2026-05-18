//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 384/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk384<F: Float>(t128: F, t1686: F, t933: F, t1: F, t436: F, t431: F, t1659: F, t432: F, t925: F, t435: F, t95: F) -> (F, F, F, F, F, F) {
    let t1687 = t1686 * t128;
    let t1689 = F::new(0.16322666666666666) * t1687 * t933;
    let t1690 = t436 * t1;
    let t1691 = t431 * t1690;
    let t1692 = t1691 * t1659;
    let t1695 = F::new(0.3264533333333333) * t432 * t925;
    let t1697 = F::new(1.0) / t435 / t95;
    (t1687, t1689, t1691, t1692, t1695, t1697)
}
