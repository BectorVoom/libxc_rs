//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 919/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk919<F: Float>(t156: F, t4195: F, t602: F, t1: F, t1185: F, t119: F, t603: F, t1631: F, t4204: F, t4183: F, t1634: F, t474: F) -> (F, F, F, F, F, F) {
    let t10675 = F::cast_from(0.4328416544945937_f64) * t602 * t156 * t4195;
    let t10682 = t1185 * t1;
    let t10685 = F::cast_from(2.8503734567901235e-05_f64) * t10682 * t119 * t603;
    let t10688 = t1631 * t4204;
    let t10690 = t1631 * t4183;
    let t10694 = F::cast_from(0.38474813732852775_f64) * t602 * t474 * t1634;
    (t10675, t10682, t10685, t10688, t10690, t10694)
}
