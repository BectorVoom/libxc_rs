//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1119/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1119<F: Float>(t2523: F, t933: F, t2517: F, t325: F, t6666: F, t21: F, t5: F, t8363: F, t1333: F, t35: F, t331: F, t6802: F, t1351: F, t1349: F, t15782: F, t10092: F, t10098: F, t10115: F, t1371: F, t16365: F, t1943: F, t1948: F, t2061: F, t589: F) -> (F, F, F, F, F) {
    let t16370 = t933 * t2523;
    let t16372 = t933 * t2517;
    let t16374 = t325 * t6666;
    let t16377 = t21 * t5 * t8363;
    let t16378 = t1333 * t35;
    let t16382 = t331 * t6802;
    let t16384 = t1351 * t35;
    let t16389 = t15782 * t1349 * t16384;
    let t16391 = 0.03199259259259259 * t10092 - 0.010664197530864198 * t10098 - 0.015996296296296297 * t10115 + 0.008888888888888889 * t2061 * t1371 * t1943 + 0.014814814814814815 * t16365 - 0.05333333333333334 * t2061 * t589 * t1948 - 0.007407407407407408 * t16370 - 0.0024691358024691358 * t16372 + 0.023994444444444443 * t16374 + 0.10666666666666667 * t16377 * t589 * t16378 + 0.008888888888888889 * t16382 - 0.017777777777777778 * t16377 * t1371 * t16384 - 0.19195555555555555 * t16389;
    (t16374, t16377, t16378, t16389, t16391)
}
