//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta398 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1758;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1759;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta398(t17376: f64, t3599: f64, t3704: f64, t5274: f64, t1285: f64, t17395: f64, t1032: f64, t5216: f64, t1246: f64, t12916: f64, t5353: f64, t3718: f64, t5347: f64, t1781: f64, t697: f64, t1222: f64, t5284: f64, t73: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17572, t17593, t17605) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1758(t17376, t3599, t3704, t5274, t1285, t17395);
        let (t17608, t17609, t17617, t17619, t17620, t17622, t17628, t17629, t17633) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1759(t1032, t5216, t1246, t12916, t5353, t3718, t5347, t1781, t697, t1222, t5284, t73);
    (t17572, t17593, t17605, t17608, t17609, t17617, t17619, t17620, t17622, t17628, t17629, t17633)
}
