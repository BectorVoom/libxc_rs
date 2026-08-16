//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta354 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1290;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1291;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1292;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1293;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1294;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta354(t2501: f64, t39494: f64, t2698: f64, t685: f64, t684: f64, t125: f64, t123: f64, t128: f64, t121: f64, t268: f64, t8779: f64, t588: f64, t9295: f64, t2508: f64, t692: f64, t124: f64, t138: f64, t239: f64, t39490: f64, t39492: f64, t682: f64, t701: f64, t198: f64, t39419: f64, t39422: f64, t39424: f64, t39426: f64, t39429: f64, t39432: f64, t39434: f64, t39437: f64, t39439: f64, t39442: f64, t39476: f64, t39483: f64, t765: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t39495, t39497) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1290(t2501, t39494, t2698, t685);
        let (t39498, t39500, t39501) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1291(t39497, t684, t125, t2698, t123);
        let (t39506, t39508, t39510, t39512, t39515) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1292(t128, t121, t268, t8779, t588, t9295, t2508, t39494, t39497, t692, t124, t138, t239);
        let t39520 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1293(t39490, t39492, t39495, t39498, t39501, t39506, t39508, t39510, t39512, t39515, t682, t701);
        let t39521 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1294(t198, t39419, t39422, t39424, t39426, t39429, t39432, t39434, t39437, t39439, t39442, t39476, t39483, t39520, t765);
    (t39495, t39497, t39498, t39500, t39501, t39506, t39508, t39510, t39512, t39515, t39520, t39521)
}
