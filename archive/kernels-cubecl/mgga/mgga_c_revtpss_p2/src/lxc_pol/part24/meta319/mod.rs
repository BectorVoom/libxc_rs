//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta319 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1106;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1107;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1108;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta319<F: Float>(t4263: F, t5915: F, t1504: F, t5895: F, t10227: F, t4269: F, t5823: F, t580: F, t9342: F, t100: F, t5842: F, t1509: F, t5907: F, tau1: F, t10241: F, t4279: F, t5911: F, t108: F, t105: F, t109: F, t1507: F, t1510: F, t5902: F, t5908: F, t5912: F, t97: F, t114: F, t655: F, t10201: F, t13448: F, t21818: F, t21827: F, t22590: F, t69: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t22593, t22597, t22600, t22603, t22604, t22605, t22608, t22617) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1106::<F>(t4263, t5915, t1504, t5895, t10227, t4269, t5823, t580, t9342, t100, t5842, t1509, t5907, tau1);
        let (t22618, t22621, t22624, t22625, t22628) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1107::<F>(t10241, t22617, t4279, t5911, t22604, t108, t105, t109, t1507, t1510, t22597, t22600, t22605, t22608, t5902, t5908, t5912, t97);
        let (t22629, t22633) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1108::<F>(t114, t22628, t655, t10201, t13448, t21818, t21827, t22590, t22593, t69);
    (t22593, t22603, t22604, t22608, t22618, t22621, t22624, t22625, t22628, t22629, t22633)
}
