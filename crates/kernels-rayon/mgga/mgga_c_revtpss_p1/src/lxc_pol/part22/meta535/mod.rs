//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta535 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2337;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2338;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2339;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2340;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta535(t12744: f64, t5330: f64, t1222: f64, t1266: f64, t12853: f64, t17401: f64, t17405: f64, t17412: f64, t17417: f64, t17420: f64, t17425: f64, t17426: f64, t3689: f64, t3694: f64, t3723: f64, t5335: f64, t5340: f64, t5343: f64, t5373: f64, t127: f64, t371: f64, t5318: f64, t1235: f64, t1803: f64, t3670: f64, t3685: f64, t140: f64, t5368: f64, t3624: f64, t5436: f64, t12772: f64, t5401: f64, t3625: f64, t1214: f64, t5341: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t17429 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2337(t12744, t5330);
        let t17432 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2338(t1222, t1266, t12853, t17401, t17405, t17412, t17417, t17420, t17425, t17426, t17429, t3689, t3694, t3723, t5335, t5340, t5343, t5373);
        let (t17435, t17437, t17438, t17444, t17445, t17447, t17448) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2339(t127, t371, t5318, t1235, t1803, t3670, t3685, t5373, t140, t5368, t1222, t3624, t5436);
        let (t17451, t17453, t17454) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2340(t12772, t5401, t3625, t1214, t5341);
    (t17429, t17432, t17435, t17437, t17438, t17444, t17445, t17447, t17448, t17451, t17453, t17454)
}
