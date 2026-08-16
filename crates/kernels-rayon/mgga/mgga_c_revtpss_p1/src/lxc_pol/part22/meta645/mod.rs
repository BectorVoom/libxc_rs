//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta645 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2587;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2588;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2589;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta645(t20567: f64, t448: f64, t17092: f64, t5068: f64, t16840: f64, t5109: f64, t1149: f64, t6439: f64, t3433: f64, t1733: f64, t5104: f64, t3384: f64, t6474: f64, t12248: f64, t12297: f64, t12397: f64, t16706: f64, t16708: f64, t17010: f64, t17011: f64, t20283: f64, t20285: f64, t20287: f64, t20290: f64, t20295: f64, t20300: f64, t20304: f64, t20308: f64, t20312: f64, t20315: f64, t20320: f64, t12511: f64, t17023: f64, t17026: f64, t1745: f64, t20471: f64, t3447: f64, t435: f64, t5120: f64, t5125: f64, t5143: f64, t6487: f64, t6503: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t20568, t20571, t20573, t20574, t20576, t20577, t20579) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2587(t20567, t448, t17092, t5068, t16840, t5109, t1149, t6439, t3433, t1733, t5104, t3384);
        let (t20580, t20582, t20597) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2588(t1149, t6474, t12248, t12297, t12397, t16706, t16708, t17010, t17011, t20283, t20285, t20287, t20290, t20295, t20300, t20304, t20308, t20312, t20315, t20320);
        let t20602 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2589(t12511, t17023, t17026, t1745, t20471, t20568, t20571, t20573, t20576, t20579, t20582, t20597, t3447, t435, t5120, t5125, t5143, t6487, t6503);
    (t20568, t20571, t20573, t20574, t20576, t20577, t20579, t20580, t20582, t20597, t20602)
}
