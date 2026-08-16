//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta946 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3183;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3184;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta946(t43766: f64, t44361: f64, t12916: f64, t17419: f64, t5340: f64, t45608: f64, t58919: f64, t45786: f64, t17708: f64, t45846: f64, t12975: f64, t1803: f64, t225: f64, t56412: f64, t480: f64, t12984: f64, t5323: f64, t17390: f64, t3718: f64, t17500: f64, t372: f64, t13142: f64, t56878: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t58983, t58997, t59001, t59011, t59017, t59025) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3183(t43766, t44361, t12916, t17419, t5340, t45608, t58919, t45786, t17708, t45846, t12975, t1803);
        let (t59032, t59033, t59040, t59043, t59062, t59066) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3184(t225, t56412, t480, t12984, t5323, t12916, t17390, t3718, t17500, t372, t13142, t56878);
    (t58983, t58997, t59001, t59011, t59017, t59025, t59032, t59033, t59040, t59043, t59062, t59066)
}
