//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 624/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk624<F: Float>(t133: F, t5506: F, t5521: F, t1904: F, t285: F, t477: F, t281: F, t1128: F, t780: F, t1798: F, t390: F, t40: F, t1799: F, t344: F, t1775: F, t339: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t5660 = t133 * t5506;
    let t5663 = 1.1495033333333333 * t133 * t5521;
    let t5677 = t1904 * t477 * t285;
    let t5679 = 0.02394846802050922 * t281 * t5677;
    let t5681 = t780 * t1128 * t285;
    let t5682 = t281 * t5681;
    let t5685 = t1798 * t390;
    let t5686 = t40 * t5685;
    let t5687 = 2.0 * t5686;
    let t5688 = t344 * t1799;
    let t5689 = 8.0 * t5688;
    let t5690 = t339 * t1775;
    (t5660, t5663, t5677, t5679, t5681, t5682, t5685, t5686, t5687, t5688, t5689, t5690)
}
