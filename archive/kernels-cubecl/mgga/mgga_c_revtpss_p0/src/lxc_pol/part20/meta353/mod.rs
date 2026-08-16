//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta353 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1284;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1285;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1286;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1287;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1288;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1289;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta353<F: Float>(t2258: F, t2: F, t588: F, t22: F, t30: F, t33: F, zeta_threshold: F, t45: F, t57: F, t10326: F, t10472: F, t2251: F, t2299: F, t39443: F, t633: F, t766: F, t80: F, t10481: F, t2306: F, t637: F, t770: F, t83: F, t2576: F, t2565: F, t701: F, t121: F, t4: F, t131: F, t268: F, t8779: F, t9282: F, t239: F, t2456: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t39449 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1284::<F>(t2258);
        let (t39454, t39455, t39456) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1285::<F>(t2, t588, t22);
        let t39457 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1286::<F>(t30, t33, t39456, zeta_threshold);
        let (t39461, t39474) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1287::<F>(t45, t57, t10326, t10472, t2251, t2258, t2299, t39443, t39449, t39457, t633, t766, t80, t10481, t2306, t637, t770, t83, zeta_threshold);
        let (t39476, t39480, t39483) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1288::<F>(t39461, t39474, t2576, t2565, t701);
        let (t39484, t39490, t39492, t39494) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1289::<F>(t121, t4, t131, t268, t8779, t588, t9282, t239, t2456);
    (t39449, t39454, t39455, t39456, t39457, t39476, t39480, t39483, t39484, t39490, t39492, t39494)
}
