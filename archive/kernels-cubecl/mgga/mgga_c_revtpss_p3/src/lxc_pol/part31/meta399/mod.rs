//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta399 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1442;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1443;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta399<F: Float>(t12879: F, t1715: F, t247: F, t1261: F, t12916: F, t5342: F, t5340: F, t127: F, t371: F, t5318: F, t1235: F, t3685: F, t5373: F, t140: F, t5368: F, t1222: F, t3624: F, t5436: F, t12772: F, t5401: F, t3625: F, t1214: F, t1250: F, t3698: F, t5047: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t17417, t17425, t17437, t17444) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1442::<F>(t12879, t1715, t247, t1261, t12916, t5342, t5340, t127, t371, t5318, t1235, t3685, t5373);
        let (t17447, t17448, t17453, t17459, t17472) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1443::<F>(t140, t5368, t1222, t3624, t5436, t12772, t5401, t3625, t1214, t1250, t3698, t5047);
    (t17417, t17425, t17437, t17444, t17447, t17448, t17453, t17459, t17472)
}
