//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta90 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk527;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk528;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk529;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk530;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta90(t256: f64, t866: f64, t225: f64, t2435: f64, t871: f64, t785: f64, t870: f64, t2439: f64, t123: f64, t212: f64, t676: f64, t822: f64, t136: f64, t251: f64, t2457: f64, t2710: f64, t786: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2769, t2770, t2776, t2777) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk527(t256, t866, t225, t2435, t871, t785);
        let (t2778, t2780, t2782) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk528(t2777, t870, t2439, t123, t212, t676);
        let t2783 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk529(t225, t822);
        let (t2793, t2796, t2797, t2798) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk530(t136, t251, t2457, t2710, t2783, t786);
    (t2769, t2770, t2776, t2777, t2778, t2780, t2782, t2783, t2793, t2796, t2797, t2798)
}
