//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1004/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1004(t11687: f64, t11691: f64, t11695: f64, t11699: f64, t11701: f64, t11703: f64, t11705: f64, t11707: f64, t11709: f64, t11711: f64, t11713: f64, t11715: f64, t11718: f64, t11721: f64, t11724: f64, t11726: f64, t11729: f64, t11731: f64, t11748: f64, t1268: f64, t2061: f64, t25: f64, t3516: f64, t538: f64, t9761: f64) -> f64 {
    let t11750 = -0.0022222222222222222_f64 * t25 * t1268 * t11687 - 0.013333333333333334_f64 * t2061 * t1268 * t11691 - 0.007407407407407408_f64 * t11695 - 0.8638_f64 * t11699 + 0.07198333333333333_f64 * t11703 + 0.4319_f64 * t11707 + 0.09597777777777777_f64 * t11709 + 0.023994444444444443_f64 * t11711 + 0.03999074074074074_f64 * t11713 + 0.5278777777777778_f64 * t11715 - 0.023994444444444443_f64 * t11718 - 0.14396666666666666_f64 * t11721 - 0.10664197530864197_f64 * t11726 - 0.23994444444444443_f64 * t11731 - 0.006913580246913581_f64 * t25 * t9761 * t11724 - 0.017777777777777778_f64 * t2061 * t3516 * t11729 + 0.013333333333333334_f64 * t25 * t538 * t11701 + 0.08_f64 * t2061 * t538 * t11705 + 0.8638_f64 * t11748;
    t11750
}
