//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta701 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2714;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta701(t14239: f64, t5741: f64, t6844: f64, t72: f64, t686: f64, t4101: f64, t6874: f64, t10098: f64, t10102: f64, t10109: f64, t10114: f64, t14218: f64, t14221: f64, t14227: f64, t14229: f64, t14233: f64, t14241: f64, t14243: f64, t22005: f64, t5675: f64, t5745: f64) -> (f64, f64, f64, f64, f64) {
        let (t22331, t22332, t22335, t22336, t22344) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2714(t14239, t5741, t6844, t72, t686, t4101, t6874, t10098, t10102, t10109, t10114, t14218, t14221, t14227, t14229, t14233, t14241, t14243, t22005, t5675, t5745);
    (t22331, t22332, t22335, t22336, t22344)
}
