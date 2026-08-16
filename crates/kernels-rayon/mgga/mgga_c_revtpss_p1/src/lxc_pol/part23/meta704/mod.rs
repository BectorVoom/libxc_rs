//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta704 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2454;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2455;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta704(t72: f64, t9940: f64, t245: f64, t3951: f64, t3964: f64, t9732: f64, t1353: f64, t9994: f64, t136: f64, t4010: f64, t220: f64, t2482: f64, t27: f64, t9991: f64, t1389: f64, t40604: f64, t10111: f64, t22: f64, t4092: f64, t39515: f64, t4083: f64, t10043: f64, t9303: f64, t14192: f64, t555: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t47247, t47248, t47262, t47264, t47273, t47274, t47293) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2454(t72, t9940, t245, t3951, t3964, t9732, t1353, t9994, t136, t4010, t220, t2482, t27, t9991);
        let (t47337, t47348, t47351, t47352, t47371) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2455(t1389, t3964, t40604, t10111, t22, t4092, t39515, t4083, t10043, t9303, t14192, t555);
    (t47247, t47248, t47262, t47264, t47273, t47274, t47293, t47337, t47348, t47351, t47352, t47371)
}
