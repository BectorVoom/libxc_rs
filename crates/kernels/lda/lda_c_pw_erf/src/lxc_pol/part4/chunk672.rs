//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 672/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk672<F: Float>(t479: F, t695: F, t1: F, t1750: F, t726: F, t1755: F, t116: F, t717: F, t732: F, t731: F, t1184: F, t1753: F, t279: F, t1752: F, t1746: F, t1759: F) -> (F, F, F, F, F, F, F, F) {
    let t4279 = 0.1890324433388467 * t695 * t479;
    let t4291 = t726 * t1750 * t1;
    let t4292 = t4291 * t1755;
    let t4295 = t732 * t717 * t116;
    let t4296 = t731 * t4295;
    let t4299 = t1753 * t1184 * t279;
    let t4300 = t1752 * t4299;
    let t4304 = t1759 * t1746;
    (t4279, t4291, t4292, t4295, t4296, t4299, t4300, t4304)
}
