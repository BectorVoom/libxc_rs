//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta353 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1284;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1285;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1286;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1287;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1288;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1289;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta353(t2258: f64, t2: f64, t588: f64, t22: f64, t30: f64, t33: f64, zeta_threshold: f64, t45: f64, t57: f64, t10326: f64, t10472: f64, t2251: f64, t2299: f64, t39443: f64, t633: f64, t766: f64, t80: f64, t10481: f64, t2306: f64, t637: f64, t770: f64, t83: f64, t2576: f64, t2565: f64, t701: f64, t121: f64, t4: f64, t131: f64, t268: f64, t8779: f64, t9282: f64, t239: f64, t2456: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t39449 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1284(t2258);
        let (t39454, t39455, t39456) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1285(t2, t588, t22);
        let t39457 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1286(t30, t33, t39456, zeta_threshold);
        let (t39461, t39474) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1287(t45, t57, t10326, t10472, t2251, t2258, t2299, t39443, t39449, t39457, t633, t766, t80, t10481, t2306, t637, t770, t83, zeta_threshold);
        let (t39476, t39480, t39483) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1288(t39461, t39474, t2576, t2565, t701);
        let (t39484, t39490, t39492, t39494) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1289(t121, t4, t131, t268, t8779, t588, t9282, t239, t2456);
    (t39449, t39454, t39455, t39456, t39457, t39476, t39480, t39483, t39484, t39490, t39492, t39494)
}
