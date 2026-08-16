//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta501 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1987;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1988;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta501(t20800: f64, t5341: f64, t3720: f64, t5333: f64, t1263: f64, t6587: f64, t1122: f64, t1042: f64, t3172: f64, t6624: f64, t1247: f64, t1032: f64, t6564: f64, t1246: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t20801, t20802, t20805, t20806, t20809, t20810, t20811, t20816, t20817, t20819) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1987(t20800, t5341, t3720, t5333, t1263, t6587, t1122, t1042, t3172, t6624, t1247, t1032, t6564);
        let t20820 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1988(t1246, t20819);
    (t20801, t20802, t20805, t20806, t20809, t20810, t20811, t20816, t20817, t20819, t20820)
}
