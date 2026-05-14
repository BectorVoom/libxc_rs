//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 788/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk788<F: Float>(t2990: F, t1775: F, t344: F, t1799: F, t339: F, t3002: F, t3004: F, t1: F, t1798: F, t397: F, t3010: F, t3018: F, t2995: F, t3000: F, t3009: F, t3016: F, t3118: F, t3121: F, t3125: F, t3155: F) -> (F, F, F, F, F, F, F) {
    let t5694 = 34.631511798751724 * t2990;
    let t5695 = t344 * t1775;
    let t5696 = 8.0 * t5695;
    let t5697 = t339 * t1799;
    let t5698 = 8.0 * t5697;
    let t5699 = 0.0001831155503675316 * t3002;
    let t5700 = 0.0004883081343134176 * t3004;
    let t5701 = t1798 * t1;
    let t5702 = t5701 * t397;
    let t5703 = 0.0003662311007350632 * t5702;
    let t5704 = 4.0 * t3010;
    let t5705 = 2.0 * t3018;
    let t5706 = -t5694 - t5696 + t5698 + t2995 - t3000 - t5699 + t5700 - t3009 - t5703 + t5704 + t3016 + t5705 + t3155 + t3118 - t3121 + t3125;
    (t5695, t5698, t5701, t5702, t5703, t5704, t5706)
}
