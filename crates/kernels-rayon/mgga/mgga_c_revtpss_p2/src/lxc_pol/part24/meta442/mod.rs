//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta442 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1399;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1400;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta442(t220: f64, t47273: f64, t2482: f64, t27: f64, t9991: f64, t1389: f64, t3964: f64, t40604: f64, t39515: f64, t4083: f64, t14192: f64, t555: f64, t786: f64, t1432: f64, t1433: f64, t39497: f64, t10111: f64, t1428: f64, t588: f64, t10022: f64, t2453: f64, t268: f64, t39644: f64, t546: f64, t8779: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t47274, t47293, t47337, t47351, t47371) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1399(t220, t47273, t2482, t27, t9991, t1389, t3964, t40604, t39515, t4083, t14192, t555);
        let (t47372, t47395, t47417, t47429, t47442) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1400(t47371, t786, t1432, t1433, t39497, t10111, t1428, t588, t10022, t2453, t268, t39644, t546, t555, t8779);
    (t47274, t47293, t47337, t47351, t47372, t47395, t47417, t47429, t47442)
}
