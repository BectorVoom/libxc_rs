//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta448 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1711;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1712;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta448(t3368: f64, t5277: f64, t1042: f64, t3704: f64, t5274: f64, t1774: f64, t3588: f64, t1250: f64, t3720: f64, t1285: f64, t17395: f64, t1032: f64, t5216: f64, t1246: f64, t1252: f64, t12956: f64, t12999: f64, t13012: f64, t13015: f64, t13018: f64, t3631: f64, t3647: f64, t3711: f64, t3718: f64, t5279: f64, t5304: f64) -> (f64, f64, f64, f64, f64) {
        let (t17589, t17593, t17600, t17602, t17605, t17608) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1711(t3368, t5277, t1042, t3704, t5274, t1774, t3588, t1250, t3720, t1285, t17395, t1032, t5216);
        let t17614 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1712(t1246, t17608, t1252, t12956, t12999, t13012, t13015, t13018, t17589, t17593, t17602, t17605, t3631, t3647, t3711, t3718, t5279, t5304);
    (t17589, t17600, t17602, t17608, t17614)
}
