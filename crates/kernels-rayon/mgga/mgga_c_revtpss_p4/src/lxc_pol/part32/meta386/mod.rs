//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta386 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1347;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1348;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta386(t12879: f64, t1715: f64, t247: f64, t1261: f64, t12916: f64, t5342: f64, t5340: f64, t127: f64, t371: f64, t5318: f64, t1235: f64, t3685: f64, t5373: f64, t140: f64, t5368: f64, t1222: f64, t3624: f64, t5436: f64, t12772: f64, t5401: f64, t3625: f64, t1214: f64, t1250: f64, t3698: f64, t5047: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17417, t17425, t17437, t17444) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1347(t12879, t1715, t247, t1261, t12916, t5342, t5340, t127, t371, t5318, t1235, t3685, t5373);
        let (t17447, t17448, t17453, t17459, t17472) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1348(t140, t5368, t1222, t3624, t5436, t12772, t5401, t3625, t1214, t1250, t3698, t5047);
    (t17417, t17425, t17437, t17444, t17447, t17448, t17453, t17459, t17472)
}
