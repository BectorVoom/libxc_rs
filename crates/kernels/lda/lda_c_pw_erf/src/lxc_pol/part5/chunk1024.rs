//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1024/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1024<F: Float>(t331: F, t6510: F, t6513: F, t6516: F, t2488: F, t933: F, t2491: F, t10030: F, t6743: F, t6749: F, t565: F, t6297: F) -> (F, F, F, F, F, F, F, F) {
    let t17290 = t331 * t6510;
    let t17295 = t331 * t6513;
    let t17301 = t331 * t6516;
    let t17327 = t933 * t2488;
    let t17332 = t933 * t2491;
    let t17396 = t10030 * t6743;
    let t17398 = t10030 * t6749;
    let t17413 = t565 * t6297;
    (t17290, t17295, t17301, t17327, t17332, t17396, t17398, t17413)
}
