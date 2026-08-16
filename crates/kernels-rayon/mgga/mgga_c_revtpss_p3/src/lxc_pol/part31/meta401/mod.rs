//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta401 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1446;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1447;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta401(t1285: f64, t17395: f64, t1032: f64, t5216: f64, t1246: f64, t12916: f64, t5353: f64, t3718: f64, t5347: f64, t1781: f64, t697: f64, t1222: f64, t5284: f64, t73: f64, t17350: f64, t3767: f64, t372: f64, t5277: f64, t12865: f64, t15904: f64, t3623: f64, t13148: f64, t3172: f64, t5303: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17605, t17609, t17619, t17622, t17629) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1446(t1285, t17395, t1032, t5216, t1246, t12916, t5353, t3718, t5347, t1781, t697, t1222);
        let (t17633, t17654, t17661, t17693, t17708, t17709, t17720) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1447(t5284, t73, t17350, t3767, t372, t5277, t1285, t12865, t15904, t3623, t13148, t3172, t5303);
    (t17605, t17609, t17619, t17622, t17629, t17633, t17654, t17661, t17693, t17708, t17709, t17720)
}
