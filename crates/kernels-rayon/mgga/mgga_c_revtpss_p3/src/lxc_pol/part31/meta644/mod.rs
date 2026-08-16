//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta644 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2104;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta644(t27375: f64, t98658: f64, t1468: f64, t4343: f64, t5962: f64, t605: f64, t6075: f64, t775: f64, t25207: f64, t1583: f64, t580: f64, t98631: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t105906, t105909, t105919, t105923, t105924, t105928) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2104(t27375, t98658, t1468, t4343, t5962, t605, t6075, t775, t25207, t1583, t580, t98631);
    (t105906, t105909, t105919, t105923, t105924, t105928)
}
