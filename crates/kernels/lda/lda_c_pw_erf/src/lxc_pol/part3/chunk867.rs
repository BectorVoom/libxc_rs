//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 867/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk867<F: Float>(t1217: F, t1231: F, t3701: F, t668: F, t1410: F, t1433: F, t1426: F, t635: F, t645: F, t1210: F, t646: F, t3940: F, t656: F, t3943: F, t1416: F, t1419: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11105 = t1231 * t1217;
    let t11107 = t3701 * t668;
    let t11153 = 4.0 / 9.0 * t1433 * t1410;
    let t11156 = 0.05402469135802469 * t645 * t635 * t1426;
    let t11159 = 0.05402469135802469 * t645 * t1210 * t646;
    let t11160 = t3940 * t656;
    let t11162 = t3943 * t656;
    let t11164 = t1416 * t1410;
    let t11166 = t1419 * t1410;
    (t11105, t11107, t11153, t11156, t11159, t11160, t11162, t11164, t11166)
}
