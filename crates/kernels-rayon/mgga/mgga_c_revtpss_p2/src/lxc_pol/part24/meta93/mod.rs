//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta93 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk537;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk538;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk539;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta93(t2922: f64, t275: f64, t290: f64, t2846: f64, t307: f64, t944: f64, t302: f64, t2904: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2923, t2924) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk537(t2922, t275);
        let (t2925, t2926) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk538(t290);
        let (t2930, t2941, t2942, t2943, t2950, t2957, t2966) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk539(t2846, t307, t944, t302, t2904);
    (t2923, t2924, t2925, t2926, t2930, t2941, t2942, t2943, t2950, t2957, t2966)
}
