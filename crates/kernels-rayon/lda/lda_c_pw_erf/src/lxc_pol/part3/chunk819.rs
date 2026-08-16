//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 819/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk819(t1798: f64, t390: f64, t40: f64, t1799: f64, t344: f64, t1775: f64, t339: f64, t2748: f64, t2752: f64, t2755: f64, t2759: f64, t2761: f64, t2944: f64, t2950: f64, t2989: f64, t4406: f64, t4409: f64, t4411: f64, t4413: f64, t4420: f64) -> (f64, f64, f64, f64, f64) {
    let t5685 = t1798 * t390;
    let t5686 = t40 * t5685;
    let t5687 = 2.0_f64 * t5686;
    let t5688 = t344 * t1799;
    let t5689 = 8.0_f64 * t5688;
    let t5690 = t339 * t1775;
    let t5691 = 8.0_f64 * t5690;
    let t5692 = t4406 + t4409 + t4411 + t4413 - t2748 + t2752 - t2755 + t2759 - t2761 - t2944 + t2950 + t4420 + t5687 - t5689 + t5691 - t2989;
    (t5685, t5686, t5688, t5690, t5692)
}
