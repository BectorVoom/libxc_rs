//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 872/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk872<F: Float>(t14385: F, t1910: F, t2910: F, t485: F, t1124: F, t1904: F, t483: F, t1131: F, t5470: F, t1191: F, t780: F, t1138: F, t1597: F, t5932: F, t717: F, t2916: F, t5466: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t14386 = 0.01185233419734569 * t14385;
    let t14388 = t1910 * t2910 * t485;
    let t14392 = t1124 * t1904 * t483 * t485;
    let t14393 = 0.01975389032890948 * t14392;
    let t14395 = t5470 * t1131 * t485;
    let t14397 = t1191 * t780;
    let t14399 = t14397 * t1138 * t1597;
    let t14401 = t5932 * t485;
    let t14403 = t717 * t1904;
    let t14405 = t14403 * t1138 * t1597;
    let t14406 = 0.0014862827083471494 * t14405;
    let t14408 = t5466 * t2916 * t1597;
    (t14386, t14388, t14393, t14395, t14397, t14399, t14401, t14403, t14406, t14408)
}
