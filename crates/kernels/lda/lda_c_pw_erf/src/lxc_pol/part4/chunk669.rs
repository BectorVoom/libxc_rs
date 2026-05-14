//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 669/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk669<F: Float>(t1638: F, t4199: F, t1627: F, t1631: F, t156: F, t1634: F, t602: F, t603: F, t635: F, t1620: F, t226: F, t695: F, t1612: F, t230: F, t598: F, t610: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t4201 = 0.03354522822333102 * t1638 * t4199;
    let t4202 = t1631 * t1627;
    let t4204 = t156 * t1634;
    let t4206 = 0.21642082724729686 * t602 * t4204;
    let t4207 = t635 * t603;
    let t4209 = 0.011181742741110338 * t1638 * t4207;
    let t4215 = 4.0 * t226 * t1620;
    let t4217 = 0.0011033703703703704 * t695 * t603;
    let t4218 = t1612 * t230;
    let t4220 = t598 * t610;
    (t4201, t4202, t4204, t4206, t4207, t4209, t4215, t4217, t4218, t4220)
}
