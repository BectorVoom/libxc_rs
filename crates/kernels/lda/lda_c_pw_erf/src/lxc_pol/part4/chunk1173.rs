//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1173/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1173<F: Float>(t2488: F, t933: F, t2491: F, t11848: F, t11851: F, t11855: F, t11861: F, t1268: F, t15825: F, t15887: F, t15890: F, t15893: F, t15896: F, t15899: F, t16405: F, t1966: F, t2061: F, t9891: F) -> (F,) {
    let t17327 = t933 * t2488;
    let t17332 = t933 * t2491;
    let t17348 = -0.0024691358024691358 * t17327 - 0.008888888888888889 * t2061 * t1268 * t1966 + 0.014814814814814815 * t17332 - 1.135737037037037 * t11848 - 0.010664197530864198 * t15887 - 0.09597777777777777 * t15890 + 0.2879333333333333 * t15893 - 1.7276 * t15896 + 1.1517333333333333 * t15899 + 0.017777777777777778 * t11851 + 0.10666666666666667 * t16405 * t11855 * t15825 - 0.023703703703703703 * t16405 * t11861 * t15825 + 0.014814814814814815 * t9891;
    (t17348,)
}
