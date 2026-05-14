//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1172/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1172<F: Float>(t331: F, t6516: F, t11818: F, t15836: F, t15838: F, t15842: F, t15846: F, t15853: F, t25: F, t538: F, t9824: F, t9828: F, t9832: F, t9847: F, t9866: F, t11829: F, t11834: F, t11846: F, t15848: F, t15850: F, t15855: F, t15860: F, t15865: F, t15870: F, t15874: F, t15879: F, t15883: F) -> (F, F) {
    let t17301 = t331 * t6516;
    let t17312 = 0.02666666666666667 * t25 * t538 * t15853 - 0.017777777777777778 * t17301 + t9824 + 0.047988888888888886 * t11818 - 0.015996296296296297 * t9828 - 0.010664197530864198 * t9832 + 0.07464938271604939 * t9847 + 0.03199259259259259 * t9866 - 0.09597777777777777 * t15836 - 1.0557555555555556 * t15838 - 0.21595 * t15842 - 0.8638 * t15846;
    let t17326 = -0.047988888888888886 * t15848 + 0.015996296296296297 * t15850 + 0.14396666666666666 * t15855 + 0.07198333333333333 * t15860 + 0.14396666666666666 * t15865 - 0.047988888888888886 * t15870 - 0.023994444444444443 * t15874 - 0.03999074074074074 * t15879 + 0.8638 * t15883 + 0.03950617283950617 * t11829 + 0.07464938271604939 * t11834 - 0.2725925925925926 * t11846;
    (t17312, t17326)
}
