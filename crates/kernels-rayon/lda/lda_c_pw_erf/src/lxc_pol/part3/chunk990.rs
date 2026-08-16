//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 990/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk990(t1704: f64, t2765: f64, t756: f64, t343: f64, t780: f64, t159: f64, t285: f64, t4437: f64, t477: f64, t10847: f64, t10852: f64, t1726: f64, t1729: f64, t2764: f64, t4430: f64, t454: f64, t5925: f64, t777: f64, t8768: f64, t8822: f64, t8825: f64, t8827: f64, t8831: f64, t8834: f64, t8838: f64, t8842: f64, t8845: f64) -> (f64, f64) {
    let t11543 = t2765 * t756 * t1704;
    let t11546 = t343 * t780;
    let t11548 = t11546 * t159 * t285;
    let t11551 = t4437 * t477 * t285;
    let t11556 = 0.585406996056892_f64 * t8822 + t8825 + 0.012203831437512505_f64 * t8827 + t8831 - 0.020146007452401596_f64 * t8834 - t8838 + t8842 + 0.004067943812504169_f64 * t8845 - 6.0_f64 * t8768 * t4430 + 18.0_f64 * t1729 * t1726 * t454 * t5925 - 3.0_f64 * t2764 * t11543 - 0.006715335817467199_f64 * t11548 + 0.004067943812504169_f64 * t11551 - t777 * t10847 - 3.0_f64 * t777 * t10852;
    (t11546, t11556)
}
