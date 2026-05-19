//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1004/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1004<F: Float>(t11687: F, t11691: F, t11695: F, t11699: F, t11701: F, t11703: F, t11705: F, t11707: F, t11709: F, t11711: F, t11713: F, t11715: F, t11718: F, t11721: F, t11724: F, t11726: F, t11729: F, t11731: F, t11748: F, t1268: F, t2061: F, t25: F, t3516: F, t538: F, t9761: F) -> F {
    let t11750 = -F::cast_from(0.0022222222222222222_f64) * t25 * t1268 * t11687 - F::cast_from(0.013333333333333334_f64) * t2061 * t1268 * t11691 - F::cast_from(0.007407407407407408_f64) * t11695 - F::new(0.8638) * t11699 + F::cast_from(0.07198333333333333_f64) * t11703 + F::new(0.4319) * t11707 + F::cast_from(0.09597777777777777_f64) * t11709 + F::cast_from(0.023994444444444443_f64) * t11711 + F::cast_from(0.03999074074074074_f64) * t11713 + F::cast_from(0.5278777777777778_f64) * t11715 - F::cast_from(0.023994444444444443_f64) * t11718 - F::cast_from(0.14396666666666666_f64) * t11721 - F::cast_from(0.10664197530864197_f64) * t11726 - F::cast_from(0.23994444444444443_f64) * t11731 - F::cast_from(0.006913580246913581_f64) * t25 * t9761 * t11724 - F::cast_from(0.017777777777777778_f64) * t2061 * t3516 * t11729 + F::cast_from(0.013333333333333334_f64) * t25 * t538 * t11701 + F::new(0.08) * t2061 * t538 * t11705 + F::new(0.8638) * t11748;
    t11750
}
