//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 890/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk890<F: Float>(t2438: F, t925: F, t2434: F, t325: F, t6561: F, t6504: F, t4606: F, t6507: F, t6532: F, t348: F, t739: F, t6501: F, t6535: F, t6538: F, t6541: F, t1251: F, t5992: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t15777 = t925 * t2438;
    let t15779 = t925 * t2434;
    let t15788 = t325 * t6561;
    let t15798 = t325 * t6504;
    let t15800 = t4606 * t6507;
    let t15820 = t325 * t6532;
    let t15824 = t739 * t348;
    let t15836 = t325 * t6501;
    let t15838 = t4606 * t6535;
    let t15848 = t325 * t6538;
    let t15850 = t325 * t6541;
    let t15852 = t1251 * t5992;
    (t15777, t15779, t15788, t15798, t15800, t15820, t15824, t15836, t15838, t15848, t15850, t15852)
}
