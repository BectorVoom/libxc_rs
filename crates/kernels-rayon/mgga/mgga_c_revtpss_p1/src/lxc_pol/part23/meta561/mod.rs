//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta561 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2126;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2127;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2128;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta561(t580: f64, t9342: f64, t100: f64, t5842: f64, t1509: f64, t5907: f64, t10241: f64, t4279: f64, t5911: f64, t108: f64, t105: f64, t109: f64, t1507: f64, t1510: f64, t22597: f64, t22600: f64, t5902: f64, t5908: f64, t5912: f64, t97: f64, tau1: f64, t114: f64, t655: f64, t10201: f64, t13448: f64, t21818: f64, t21827: f64, t22590: f64, t22593: f64, t69: f64, t508: f64, t1501: f64, t5883: f64, t10271: f64, t10273: f64, t10275: f64, t10278: f64, t10280: f64, t10282: f64, t10284: f64, t10287: f64, t10289: f64, t10291: f64, t10295: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22603, t22604, t22605, t22608, t22617, t22624, t22628) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2126(t580, t9342, t100, t5842, t1509, t5907, t10241, t4279, t5911, t108, t105, t109, t1507, t1510, t22597, t22600, t5902, t5908, t5912, t97, tau1);
        let (t22629, t22633) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2127(t114, t22628, t655, t10201, t13448, t21818, t21827, t22590, t22593, t69);
        let (t22634, t22639, t22648) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2128(t22633, t508, t1501, t5883, t10271, t10273, t10275, t10278, t10280, t10282, t10284, t10287, t10289, t10291, t10295);
    (t22603, t22604, t22605, t22608, t22617, t22624, t22628, t22629, t22633, t22634, t22639, t22648)
}
