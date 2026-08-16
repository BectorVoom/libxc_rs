//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta635 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2333;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2334;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2335;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2336;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2337;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta635(t39497: f64, t684: f64, t125: f64, t2698: f64, t123: f64, t128: f64, t121: f64, t268: f64, t8779: f64, t588: f64, t9295: f64, t2508: f64, t39494: f64, t692: f64, t124: f64, t138: f64, t239: f64, t39490: f64, t39492: f64, t39495: f64, t682: f64, t701: f64, t2566: f64, t9274: f64, t2584: f64, t9311: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t39498, t39500, t39501) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2333(t39497, t684, t125, t2698, t123);
        let (t39506, t39508, t39510, t39512, t39515) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2334(t128, t121, t268, t8779, t588, t9295, t2508, t39494, t39497, t692, t124, t138, t239);
        let t39520 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2335(t39490, t39492, t39495, t39498, t39501, t39506, t39508, t39510, t39512, t39515, t682, t701);
        let (t39525, t39528) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2336(t2566, t701, t9274);
        let t39531 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2337(t2584, t39525, t9311);
    (t39498, t39500, t39501, t39506, t39508, t39510, t39512, t39515, t39520, t39525, t39528, t39531)
}
