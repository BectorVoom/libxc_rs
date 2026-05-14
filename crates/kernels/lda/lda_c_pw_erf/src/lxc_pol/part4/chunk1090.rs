//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1090/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1090<F: Float>(t1243: F, t1953: F, t1966: F, t1971: F, t503: F, t11879: F, t15823: F, t15825: F, t11867: F, t11834: F, t11848: F, t15860: F, t15865: F, t15870: F, t15874: F, t15879: F, t15883: F, t15887: F) -> (F, F, F, F, F) {
    let t15890 = t1953 * t1243 * t1966;
    let t15893 = t1953 * t503 * t1971;
    let t15896 = t15823 * t11879 * t15825;
    let t15899 = t15823 * t11867 * t15825;
    let t15901 = -0.003778333333333333 * t15860 - 0.007556666666666666 * t15865 + 0.002518888888888889 * t15870 + 0.0012594444444444445 * t15874 + 0.002099074074074074 * t15879 - 0.04534 * t15883 - 0.003918271604938271 * t11834 + 0.059613703703703703 * t11848 + 0.000559753086419753 * t15887 + 0.005037777777777778 * t15890 - 0.015113333333333333 * t15893 + 0.09068 * t15896 - 0.06045333333333333 * t15899;
    (t15890, t15893, t15896, t15899, t15901)
}
