//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta645 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2587;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2588;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2589;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta645<F: Float>(t20567: F, t448: F, t17092: F, t5068: F, t16840: F, t5109: F, t1149: F, t6439: F, t3433: F, t1733: F, t5104: F, t3384: F, t6474: F, t12248: F, t12297: F, t12397: F, t16706: F, t16708: F, t17010: F, t17011: F, t20283: F, t20285: F, t20287: F, t20290: F, t20295: F, t20300: F, t20304: F, t20308: F, t20312: F, t20315: F, t20320: F, t12511: F, t17023: F, t17026: F, t1745: F, t20471: F, t3447: F, t435: F, t5120: F, t5125: F, t5143: F, t6487: F, t6503: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t20568, t20571, t20573, t20574, t20576, t20577, t20579) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2587::<F>(t20567, t448, t17092, t5068, t16840, t5109, t1149, t6439, t3433, t1733, t5104, t3384);
        let (t20580, t20582, t20597) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2588::<F>(t1149, t6474, t12248, t12297, t12397, t16706, t16708, t17010, t17011, t20283, t20285, t20287, t20290, t20295, t20300, t20304, t20308, t20312, t20315, t20320);
        let t20602 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2589::<F>(t12511, t17023, t17026, t1745, t20471, t20568, t20571, t20573, t20576, t20579, t20582, t20597, t3447, t435, t5120, t5125, t5143, t6487, t6503);
    (t20568, t20571, t20573, t20574, t20576, t20577, t20579, t20580, t20582, t20597, t20602)
}
