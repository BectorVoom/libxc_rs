//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta779 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2584;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2585;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta779(t45619: f64, t58919: f64, t3666: f64, t5390: f64, t43766: f64, t44361: f64, t45608: f64, t45786: f64, t12984: f64, t5323: f64, t17500: f64, t372: f64, t13142: f64, t56878: f64, t12851: f64, t1778: f64, t3766: f64, t5219: f64, t5330: f64, t3718: f64, t44546: f64, t5353: f64, t45833: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t58920, t58927, t58983, t59001, t59011, t59041, t59062) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2584(t45619, t58919, t3666, t5390, t43766, t44361, t45608, t45786, t12984, t5323, t17500, t372);
        let (t59066, t59144, t59162, t59186, t59196) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2585(t13142, t56878, t12851, t1778, t3766, t5219, t5330, t3718, t44546, t5353, t45833, t58919);
    (t58920, t58927, t58983, t59001, t59011, t59041, t59062, t59066, t59144, t59162, t59186, t59196)
}
