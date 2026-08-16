//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta590 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1885;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1886;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta590(t23237: f64, t25341: f64, t6552: f64, t23204: f64, t25216: f64, t6562: f64, t1519: f64, t212: f64, t23171: f64, t6554: f64, t23270: f64, t25038: f64, t258: f64, t4119: f64, t776: f64, t25039: f64, t2553: f64, t25040: f64, t82074: f64, t87712: f64, t25193: f64, t81591: f64, t1484: f64, t2249: f64, t606: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t87907, t87910, t87915, t87920) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1885(t23237, t25341, t6552, t23204, t25216, t6562, t1519, t212, t23171, t6554, t23270, t25038, t258, t4119, t776);
        let (t87924, t87927, t87931, t87953, t87957) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1886(t23270, t25038, t25039, t2553, t25040, t82074, t87712, t25193, t81591, t1484, t2249, t4119, t606);
    (t87907, t87910, t87915, t87920, t87924, t87927, t87931, t87953, t87957)
}
