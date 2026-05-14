//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 625/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk625<F: Float>(t1775: F, t344: F, t1799: F, t339: F, t1: F, t1798: F, t397: F, t3010: F, t3158: F, t3161: F, t3169: F, t3173: F, t1880: F, t405: F, t455: F, t5495: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t5695 = t344 * t1775;
    let t5697 = t339 * t1799;
    let t5698 = 8.0 * t5697;
    let t5701 = t1798 * t1;
    let t5702 = t5701 * t397;
    let t5703 = 0.0003662311007350632 * t5702;
    let t5704 = 4.0 * t3010;
    let t5707 = 48.0 * t3158;
    let t5708 = 80.0 * t3161;
    let t5709 = 12.0 * t3169;
    let t5711 = 32.0 * t3173;
    let t5735 = t405 * t1880;
    let t5740 = t455 * t5495;
    (t5695, t5697, t5698, t5701, t5702, t5703, t5704, t5707, t5708, t5709, t5711, t5735, t5740)
}
