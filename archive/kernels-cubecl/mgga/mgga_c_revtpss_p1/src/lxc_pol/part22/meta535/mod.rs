//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta535 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2337;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2338;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2339;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2340;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta535<F: Float>(t12744: F, t5330: F, t1222: F, t1266: F, t12853: F, t17401: F, t17405: F, t17412: F, t17417: F, t17420: F, t17425: F, t17426: F, t3689: F, t3694: F, t3723: F, t5335: F, t5340: F, t5343: F, t5373: F, t127: F, t371: F, t5318: F, t1235: F, t1803: F, t3670: F, t3685: F, t140: F, t5368: F, t3624: F, t5436: F, t12772: F, t5401: F, t3625: F, t1214: F, t5341: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t17429 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2337::<F>(t12744, t5330);
        let t17432 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2338::<F>(t1222, t1266, t12853, t17401, t17405, t17412, t17417, t17420, t17425, t17426, t17429, t3689, t3694, t3723, t5335, t5340, t5343, t5373);
        let (t17435, t17437, t17438, t17444, t17445, t17447, t17448) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2339::<F>(t127, t371, t5318, t1235, t1803, t3670, t3685, t5373, t140, t5368, t1222, t3624, t5436);
        let (t17451, t17453, t17454) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2340::<F>(t12772, t5401, t3625, t1214, t5341);
    (t17429, t17432, t17435, t17437, t17438, t17444, t17445, t17447, t17448, t17451, t17453, t17454)
}
