//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 986/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk986<F: Float>(t142: F, t5548: F, t455: F, t1704: F, t2765: F, t756: F, t343: F, t780: F, t159: F, t285: F, t4437: F, t477: F, t2783: F, t872: F, t1187: F, t2824: F, t483: F) -> (F, F, F, F, F, F, F) {
    let t11510 = t142 * t5548;
    let t11511 = t455 * t11510;
    let t11543 = t2765 * t756 * t1704;
    let t11546 = t343 * t780;
    let t11548 = t11546 * t159 * t285;
    let t11551 = t4437 * t477 * t285;
    let t11557 = t2783 * t872;
    let t11561 = t2824 * t780 * t483 * t1187;
    (t11511, t11543, t11546, t11548, t11551, t11557, t11561)
}
