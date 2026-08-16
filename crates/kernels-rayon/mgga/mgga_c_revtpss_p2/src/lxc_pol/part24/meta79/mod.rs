//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta79 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk474;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk475;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk476;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk477;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk478;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta79(t2455: f64, t2457: f64, t2454: f64, t252: f64, t867: f64, t786: f64, t215: f64, t685: f64, t788: f64, t787: f64, t206: f64, t242: f64, t240: f64, t72: f64, t225: f64, t27: f64, t823: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2458, t2460, t2464, t2465) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk474(t2455, t2457, t2454, t252, t867, t786);
        let t2470 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk475(t215, t685);
        let (t2471, t2473, t2475) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk476(t2470, t788, t787, t206, t242);
        let (t2476, t2477, t2482) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk477(t240, t2475, t72, t225, t786);
        let t2484 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk478(t2482, t27, t823);
    (t2458, t2460, t2464, t2465, t2470, t2471, t2473, t2475, t2476, t2477, t2482, t2484)
}
