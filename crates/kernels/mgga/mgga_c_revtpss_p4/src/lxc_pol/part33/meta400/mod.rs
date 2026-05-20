//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta400 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1450;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1451;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta400<F: Float>(t140: F, t5368: F, t1222: F, t3624: F, t5436: F, t12772: F, t5401: F, t3625: F, t1214: F, t1250: F, t3698: F, t5047: F, t1012: F, t13026: F, t1263: F, t5245: F, t1234: F, t5390: F, t3704: F, t5293: F, t3172: F, t5286: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t17445, t17447, t17448, t17451, t17453, t17459, t17472) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1450::<F>(t140, t5368, t1222, t3624, t5436, t12772, t5401, t3625, t1214, t1250, t3698, t5047);
        let (t17474, t17475, t17500, t17505, t17509, t17544) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1451::<F>(t1222, t17472, t1012, t13026, t1263, t5245, t1234, t5390, t3704, t5293, t3172, t5286);
    (t17445, t17447, t17448, t17451, t17453, t17459, t17474, t17475, t17500, t17505, t17509, t17544)
}
