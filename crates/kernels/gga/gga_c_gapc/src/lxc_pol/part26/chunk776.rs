//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 776/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk776<F: Float>(t9957: F, t9959: F, t3273: F, t9529: F, t1092: F, t2486: F, t7182: F, t906: F, t904: F, t3727: F, t787: F, t2588: F, t876: F, t898: F, t1033: F, t7089: F) -> (F, F, F, F, F, F, F) {
    let t9960 = t9957 * t9959;
    let t9962 = t9529 * t3273;
    let t9964 = t1092 * t2486;
    let t9966 = t7182 * t906;
    let t9967 = t904 * t9966;
    let t9969 = t3727 * t787;
    let t9970 = t2588 * t9969;
    let t9972 = t3727 * t876;
    let t9973 = t898 * t9972;
    let t9975 = t7089 * t1033;
    (t9960, t9962, t9964, t9967, t9970, t9973, t9975)
}
