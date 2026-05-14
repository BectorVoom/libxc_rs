//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1088/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1088<F: Float>(t4606: F, t6535: F, t2429: F, t945: F, t11: F, t503: F, t6417: F, t940: F, t1243: F, t325: F, t6538: F, t6541: F, t1251: F, t5992: F, t348: F, t11818: F, t15836: F, t9828: F, t9832: F, t9847: F, t9866: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t15838 = t4606 * t6535;
    let t15840 = t2429 * t945;
    let t15842 = t11 * t503 * t15840;
    let t15844 = t6417 * t940;
    let t15846 = t11 * t1243 * t15844;
    let t15848 = t325 * t6538;
    let t15850 = t325 * t6541;
    let t15852 = t1251 * t5992;
    let t15853 = t15852 * t348;
    let t15855 = t11 * t503 * t15853;
    let t15857 = -0.002518888888888889 * t11818 + 0.0008396296296296296 * t9828 + 0.000559753086419753 * t9832 - 0.003918271604938271 * t9847 - 0.0016792592592592592 * t9866 + 0.005037777777777778 * t15836 + 0.05541555555555556 * t15838 + 0.011335 * t15842 + 0.04534 * t15846 + 0.002518888888888889 * t15848 - 0.0008396296296296296 * t15850 - 0.007556666666666666 * t15855;
    (t15838, t15840, t15842, t15844, t15846, t15848, t15850, t15853, t15855, t15857)
}
