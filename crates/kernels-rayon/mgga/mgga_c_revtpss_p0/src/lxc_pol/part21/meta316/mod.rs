//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta316 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1589;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1590;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta316(t10652: f64, t231: f64, t2783: f64, t2782: f64, t10069: f64, t2786: f64, t10073: f64, t836: f64, t860: f64, t251: f64, t2645: f64, t10111: f64, t22: f64, t870: f64, t2723: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10920, t10921, t10923, t10925, t10929, t10930, t10932, t10934, t10935, t10939) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1589(t10652, t231, t2783, t2782, t10069, t2786, t10073, t836, t860, t251, t2645, t10111, t22, t870);
        let t10943 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1590(t2645, t2723);
    (t10920, t10921, t10923, t10925, t10929, t10930, t10932, t10934, t10935, t10939, t10943)
}
