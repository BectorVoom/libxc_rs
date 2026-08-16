//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta728 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2496;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2497;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta728(t49476: f64, t1358: f64, t2439: f64, t5710: f64, t785: f64, t1426: f64, t5711: f64, t786: f64, t14100: f64, t9686: f64, t1353: f64, t198: f64, t10199: f64, t1514: f64, t2289: f64, t4264: f64, t10227: f64, t97: f64, t10241: f64, t105: f64, t4288: f64, t4398: f64, t9372: f64, t1469: f64, t2608: f64, t4401: f64, t606: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t49477, t49480, t49503, t49513, t49541) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2496(t49476, t1358, t2439, t5710, t785, t1426, t5711, t786, t14100, t9686, t1353, t198);
        let (t49698, t49701, t49777, t49787, t49818, t49866, t49876) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2497(t10199, t1514, t2289, t4264, t10227, t97, t10241, t105, t4288, t4398, t9372, t1469, t2608, t4401, t606);
    (t49477, t49480, t49503, t49513, t49541, t49698, t49701, t49777, t49787, t49818, t49866, t49876)
}
