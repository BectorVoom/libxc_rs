//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta400 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1450;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1451;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta400(t140: f64, t5368: f64, t1222: f64, t3624: f64, t5436: f64, t12772: f64, t5401: f64, t3625: f64, t1214: f64, t1250: f64, t3698: f64, t5047: f64, t1012: f64, t13026: f64, t1263: f64, t5245: f64, t1234: f64, t5390: f64, t3704: f64, t5293: f64, t3172: f64, t5286: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17445, t17447, t17448, t17451, t17453, t17459, t17472) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1450(t140, t5368, t1222, t3624, t5436, t12772, t5401, t3625, t1214, t1250, t3698, t5047);
        let (t17474, t17475, t17500, t17505, t17509, t17544) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1451(t1222, t17472, t1012, t13026, t1263, t5245, t1234, t5390, t3704, t5293, t3172, t5286);
    (t17445, t17447, t17448, t17451, t17453, t17459, t17474, t17475, t17500, t17505, t17509, t17544)
}
