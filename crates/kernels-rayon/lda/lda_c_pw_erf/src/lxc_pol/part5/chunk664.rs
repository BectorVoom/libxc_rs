//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 664/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk664(t1128: f64, t285: f64, t780: f64, t281: f64, t1798: f64, t390: f64, t40: f64, t1799: f64, t344: f64, t1775: f64, t339: f64, t1: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5681 = t780 * t1128 * t285;
    let t5682 = t281 * t5681;
    let t5685 = t1798 * t390;
    let t5686 = t40 * t5685;
    let t5687 = 2.0_f64 * t5686;
    let t5688 = t344 * t1799;
    let t5689 = 8.0_f64 * t5688;
    let t5690 = t339 * t1775;
    let t5695 = t344 * t1775;
    let t5697 = t339 * t1799;
    let t5698 = 8.0_f64 * t5697;
    let t5701 = t1798 * t1;
    (t5681, t5682, t5685, t5686, t5687, t5688, t5689, t5690, t5695, t5697, t5698, t5701)
}
