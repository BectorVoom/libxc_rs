//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 906/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk906<F: Float>(t2494: F, t933: F, t331: F, t6558: F, t5021: F, t6528: F, t6519: F, t6522: F, t6525: F, t6510: F, t6513: F, t6516: F, t2488: F, t2491: F, t10030: F, t6743: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t17226 = t933 * t2494;
    let t17234 = t331 * t6558;
    let t17249 = t5021 * t6528;
    let t17272 = t331 * t6519;
    let t17274 = t5021 * t6522;
    let t17288 = t331 * t6525;
    let t17290 = t331 * t6510;
    let t17295 = t331 * t6513;
    let t17301 = t331 * t6516;
    let t17327 = t933 * t2488;
    let t17332 = t933 * t2491;
    let t17396 = t10030 * t6743;
    (t17226, t17234, t17249, t17272, t17274, t17288, t17290, t17295, t17301, t17327, t17332, t17396)
}
