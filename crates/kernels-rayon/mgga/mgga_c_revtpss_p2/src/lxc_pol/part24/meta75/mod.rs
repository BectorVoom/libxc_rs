//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta75 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk462;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk463;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk464;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta75(t2231: f64, t27: f64, t592: f64, t596: f64, t21: f64, t25: f64, t89: f64, t90: f64, t29: f64, t2: f64, t580: f64, t47: f64, t59: f64, t239: f64, t64: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2233, t2235, t2236) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk462(t2231, t27, t592, t596, t21);
        let t2237 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk463(t2236);
        let (t2239, t2246, t2247, t2255, t2275, t2282, t2289) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk464(t2237, t25, t89, t90, t29, t2, t580, t47, t59, t239, t64);
    (t2233, t2235, t2236, t2237, t2239, t2246, t2247, t2255, t2275, t2282, t2289)
}
