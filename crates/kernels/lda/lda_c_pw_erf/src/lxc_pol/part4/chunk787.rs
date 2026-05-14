//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 787/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk787<F: Float>(t1798: F, t390: F, t40: F, t1799: F, t344: F, t1775: F, t339: F, t2748: F, t2752: F, t2755: F, t2759: F, t2761: F, t2944: F, t2950: F, t2989: F, t4406: F, t4409: F, t4411: F, t4413: F, t4420: F) -> (F, F, F, F, F, F, F) {
    let t5685 = t1798 * t390;
    let t5686 = t40 * t5685;
    let t5687 = 2.0 * t5686;
    let t5688 = t344 * t1799;
    let t5689 = 8.0 * t5688;
    let t5690 = t339 * t1775;
    let t5691 = 8.0 * t5690;
    let t5692 = t4406 + t4409 + t4411 + t4413 - t2748 + t2752 - t2755 + t2759 - t2761 - t2944 + t2950 + t4420 + t5687 - t5689 + t5691 - t2989;
    (t5685, t5686, t5687, t5688, t5689, t5690, t5692)
}
