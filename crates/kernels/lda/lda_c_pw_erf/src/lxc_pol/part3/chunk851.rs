//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 851/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk851<F: Float>(t1: F, t1185: F, t119: F, t603: F, t1627: F, t4187: F, t1631: F, t4204: F, t4183: F, t1634: F, t474: F, t602: F, t1210: F, t1638: F, t1639: F, t20: F, t3945: F) -> (F, F, F, F, F, F, F, F) {
    let t10682 = t1185 * t1;
    let t10685 = 2.8503734567901235e-05 * t10682 * t119 * t603;
    let t10686 = t4187 * t1627;
    let t10688 = t1631 * t4204;
    let t10690 = t1631 * t4183;
    let t10694 = 0.38474813732852775 * t602 * t474 * t1634;
    let t10697 = 0.019878653761973935 * t1638 * t1210 * t603;
    let t10699 = t3945 * t20 * t1639;
    (t10682, t10685, t10686, t10688, t10690, t10694, t10697, t10699)
}
