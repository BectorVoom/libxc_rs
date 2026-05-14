//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 962/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk962<F: Float>(t10654: F, t1318: F, t1321: F, t156: F, t4195: F, t602: F, t1635: F, t4187: F, t1: F, t1185: F, t119: F, t603: F, t1627: F, t1631: F, t4204: F, t4183: F) -> (F, F, F, F, F, F, F, F) {
    let t10656 = t1318 * t10654 * t1321;
    let t10675 = 0.4328416544945937 * t602 * t156 * t4195;
    let t10680 = t4187 * t1635;
    let t10682 = t1185 * t1;
    let t10685 = 2.8503734567901235e-05 * t10682 * t119 * t603;
    let t10686 = t4187 * t1627;
    let t10688 = t1631 * t4204;
    let t10690 = t1631 * t4183;
    (t10656, t10675, t10680, t10682, t10685, t10686, t10688, t10690)
}
