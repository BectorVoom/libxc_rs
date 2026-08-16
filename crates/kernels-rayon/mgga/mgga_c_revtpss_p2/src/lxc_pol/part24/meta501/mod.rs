//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta501 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1505;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1506;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta501(t1558: f64, t5962: f64, t10777: f64, t14671: f64, t14686: f64, t6017: f64, t10811: f64, t23293: f64, t1544: f64, t23327: f64, t23323: f64, t14586: f64, t14931: f64, t61715: f64, t221: f64, t23148: f64, t2674: f64, t2675: f64, t23297: f64, t14923: f64, t23336: f64, t23167: f64, t243: f64, t10726: f64, t2661: f64, t2723: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t76302, t76313, t76315, t76321, t76330, t76337, t76362) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1505(t1558, t5962, t10777, t14671, t14686, t6017, t10811, t23293, t1544, t23327, t23323, t14586, t14931, t61715);
        let (t76428, t76500, t76502, t76569, t76572) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1506(t221, t23148, t2674, t2675, t10811, t23297, t14923, t23336, t23167, t243, t10726, t2661, t2723);
    (t76302, t76313, t76315, t76321, t76330, t76337, t76362, t76428, t76500, t76502, t76569, t76572)
}
