//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 944/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk944<F: Float>(t1064: F, t1799: F, t285: F, t4422: F, t477: F, t1128: F, t1896: F, t343: F, t780: F, t159: F, t4437: F, t2783: F, t872: F) -> (F, F, F, F, F, F, F) {
    let t11471 = t1064 * t1799;
    let t11472 = F::cast_from(60.0_f64) * t11471;
    let t11498 = t4422 * t477 * t285;
    let t11499 = F::cast_from(0.0017434044910732151_f64) * t11498;
    let t11501 = t1896 * t1128 * t285;
    let t11546 = t343 * t780;
    let t11548 = t11546 * t159 * t285;
    let t11551 = t4437 * t477 * t285;
    let t11557 = t2783 * t872;
    (t11472, t11499, t11501, t11546, t11548, t11551, t11557)
}
