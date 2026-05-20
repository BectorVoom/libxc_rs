//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta529 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2174;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta529<F: Float>(t1149: F, t5105: F, t3384: F, t1733: F, t3427: F, t3385: F, t5108: F, t12248: F, t3435: F, t5104: F, t3433: F, t12230: F, t1732: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t16652, t16654, t16655, t16657, t16658, t16660, t16661, t16662, t16664, t16665, t16667, t16668) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2174::<F>(t1149, t5105, t3384, t1733, t3427, t3385, t5108, t12248, t3435, t5104, t3433, t12230, t1732);
    (t16652, t16654, t16655, t16657, t16658, t16660, t16661, t16662, t16664, t16665, t16667, t16668)
}
