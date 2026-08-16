//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta403 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1454;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta403(t5284: f64, t73: f64, t17350: f64, t3767: f64, t372: f64, t5277: f64, t1285: f64, t12865: f64, t15904: f64, t3623: f64, t13148: f64, t3172: f64, t5303: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t17633, t17654, t17661, t17693, t17708, t17709, t17720) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1454(t5284, t73, t17350, t3767, t372, t5277, t1285, t12865, t15904, t3623, t13148, t3172, t5303);
    (t17633, t17654, t17661, t17693, t17708, t17709, t17720)
}
