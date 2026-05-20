//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta557 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2247;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2248;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2249;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta557<F: Float>(t12772: F, t5401: F, t3625: F, t1214: F, t5341: F, t5332: F, t3720: F, t1250: F, t5346: F, t16725: F, t5312: F, t16729: F, t1222: F, t12855: F, t12910: F, t13069: F, t17437: F, t17438: F, t17444: F, t17447: F, t17448: F, t1797: F, t3631: F, t3674: F, t140: F, t3698: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t17451, t17453, t17454, t17455, t17456, t17459, t17460, t17461, t17464, t17467) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2247::<F>(t12772, t5401, t3625, t1214, t5341, t5332, t3720, t1250, t5346, t16725, t5312, t16729);
        let t17470 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2248::<F>(t1222, t12855, t12910, t13069, t17437, t17438, t17444, t17447, t17448, t17453, t17456, t17461, t17464, t17467, t1797, t3631, t3674);
        let t17471 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2249::<F>(t140, t3698);
    (t17451, t17454, t17455, t17456, t17459, t17460, t17461, t17470, t17471)
}
