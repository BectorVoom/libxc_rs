//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta561 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2126;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2127;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2128;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta561<F: Float>(t580: F, t9342: F, t100: F, t5842: F, t1509: F, t5907: F, t10241: F, t4279: F, t5911: F, t108: F, t105: F, t109: F, t1507: F, t1510: F, t22597: F, t22600: F, t5902: F, t5908: F, t5912: F, t97: F, tau1: F, t114: F, t655: F, t10201: F, t13448: F, t21818: F, t21827: F, t22590: F, t22593: F, t69: F, t508: F, t1501: F, t5883: F, t10271: F, t10273: F, t10275: F, t10278: F, t10280: F, t10282: F, t10284: F, t10287: F, t10289: F, t10291: F, t10295: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t22603, t22604, t22605, t22608, t22617, t22624, t22628) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2126::<F>(t580, t9342, t100, t5842, t1509, t5907, t10241, t4279, t5911, t108, t105, t109, t1507, t1510, t22597, t22600, t5902, t5908, t5912, t97, tau1);
        let (t22629, t22633) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2127::<F>(t114, t22628, t655, t10201, t13448, t21818, t21827, t22590, t22593, t69);
        let (t22634, t22639, t22648) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2128::<F>(t22633, t508, t1501, t5883, t10271, t10273, t10275, t10278, t10280, t10282, t10284, t10287, t10289, t10291, t10295);
    (t22603, t22604, t22605, t22608, t22617, t22624, t22628, t22629, t22633, t22634, t22639, t22648)
}
