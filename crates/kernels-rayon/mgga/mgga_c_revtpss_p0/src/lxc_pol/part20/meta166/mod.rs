//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta166 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk886;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk887;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk888;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk889;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta166(t2564: f64, t2567: f64, t268: f64, t675: f64, t30: f64, t525: f64, t3834: f64, t605: f64, t3833: f64, t2: f64, t22: f64, t580: f64, t2257: f64, t513: f64, t33: f64, t527: f64, t1113: f64, t3842: f64, t3841: f64, zeta_threshold: f64, t3351: f64, t516: f64, t162: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t9333 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk886(t2564, t2567, t268, t675);
        let (t9335, t9336, t9339, t9342, t9343, t9344) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk887(t30, t525, t3834, t605, t3833, t2, t22, t580);
        let (t9348, t9350, t9351, t9354, t9357) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk888(t30, t2257, t513, t9335, t9336, t9339, t9344, t33, t527, t1113, t3842, t3841, zeta_threshold);
        let t9363 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk889(t33, t3351, t516, t9350, t9351, t9354, t9357, t162, t9348, zeta_threshold);
    (t9333, t9335, t9336, t9339, t9342, t9343, t9344, t9350, t9351, t9354, t9357, t9363)
}
