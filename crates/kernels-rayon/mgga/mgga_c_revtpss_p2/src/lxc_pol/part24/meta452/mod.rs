//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta452 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1417;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1418;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta452(t2662: f64, t268: f64, t40689: f64, t4353: f64, t40710: f64, t4349: f64, t1558: f64, t231: f64, t40406: f64, t685: f64, t72: f64, t826: f64, t10760: f64, t40763: f64, t2710: f64, t4371: f64, t9732: f64, t4398: f64, t9323: f64, t4302: f64, t9586: f64, t9425: f64, t10565: f64, t1532: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t50381, t50385, t50436) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1417(t2662, t268, t40689, t4353, t40710, t4349, t1558, t231, t40406, t685, t72, t826);
        let (t50611, t50703, t50852, t50856, t50888, t50892) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1418(t10760, t40763, t4353, t2710, t4371, t9732, t4398, t9323, t4302, t9586, t9425, t10565, t1532);
    (t50381, t50385, t50436, t50611, t50703, t50852, t50856, t50888, t50892)
}
