//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta161 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk808;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk809;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta161(t1150: f64, t6438: f64, t3384: f64, t1723: f64, t3390: f64, t3394: f64, t5044: f64, t6423: f64, t6427: f64, t6431: f64) -> (f64, f64, f64, f64, f64) {
        let (t6439, t6441, t6442) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk808(t1150, t6438, t3384, t1723);
        let (t6443, t6449) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk809(t3390, t6442, t3394, t5044, t6423, t6427, t6431);
    (t6439, t6441, t6442, t6443, t6449)
}
