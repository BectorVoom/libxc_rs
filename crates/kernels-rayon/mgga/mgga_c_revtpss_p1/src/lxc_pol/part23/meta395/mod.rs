//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta395 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1752;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1753;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1754;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta395(t127: f64, t371: f64, t5318: f64, t1235: f64, t1803: f64, t3670: f64, t3685: f64, t5373: f64, t140: f64, t5368: f64, t1222: f64, t3624: f64, t5436: f64, t12772: f64, t5401: f64, t3625: f64, t1214: f64, t1250: f64, t3698: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17435, t17437, t17438, t17444, t17445, t17447, t17448) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1752(t127, t371, t5318, t1235, t1803, t3670, t3685, t5373, t140, t5368, t1222, t3624, t5436);
        let (t17451, t17453, t17459) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1753(t12772, t5401, t3625, t1214, t1250);
        let t17471 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1754(t140, t3698);
    (t17435, t17437, t17438, t17444, t17445, t17447, t17448, t17451, t17453, t17459, t17471)
}
