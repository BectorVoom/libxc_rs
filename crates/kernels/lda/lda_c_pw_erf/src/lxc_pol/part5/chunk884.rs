//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 884/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk884<F: Float>(t343: F, t865: F, t1891: F, t39: F, t1746: F, t5949: F, t5686: F, t5688: F, t5697: F, t5950: F, t5702: F, t3010: F, t3156: F, t3161: F, t3173: F, t1055: F, t5967: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t15270 = t343 * t865;
    let t15274 = t39 * t1891;
    let t15275 = 0.9598512193592288 * t15274;
    let t15296 = t5949 * t1746;
    let t15297 = 2.0538164420033334 * t15296;
    let t15306 = 6.0 * t5686;
    let t15307 = 24.0 * t5688;
    let t15311 = 24.0 * t5697;
    let t15312 = 2.464579730404 * t5950;
    let t15315 = 0.0010986933022051897 * t5702;
    let t15316 = 24.0 * t3010;
    let t15321 = 48.0 * t3156;
    let t15322 = 480.0 * t3161;
    let t15323 = 192.0 * t3173;
    let t15341 = t5967 * t1055;
    (t15270, t15275, t15297, t15306, t15307, t15311, t15312, t15315, t15316, t15321, t15322, t15323, t15341)
}
