//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta357 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1287;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1288;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta357(t12808: f64, t5330: f64, t3153: f64, t3601: f64, t1284: f64, t3555: f64, t3624: f64, t221: f64, t462: f64, t68: f64, t461: f64, t1209: f64, t3766: f64, t11772: f64, t3623: f64, t3717: f64, t1263: f64, t675: f64, t1122: f64, t247: f64, t1261: f64, t126: f64, t3617: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12809, t12810, t12832, t12853, t12854) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1287(t12808, t5330, t3153, t3601, t1284, t3555, t3624, t221, t462, t68, t461, t1209, t3766);
        let (t12855, t12865, t12866, t12879, t12882, t12884) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1288(t12854, t5330, t11772, t3623, t3717, t1263, t675, t1122, t247, t1261, t126, t3617);
    (t12809, t12810, t12832, t12853, t12855, t12865, t12866, t12879, t12882, t12884)
}
