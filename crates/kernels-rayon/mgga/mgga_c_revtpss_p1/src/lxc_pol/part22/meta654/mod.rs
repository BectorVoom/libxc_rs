//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta654 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2604;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta654(t1246: f64, t20819: f64, t1214: f64, t5819: f64, t5302: f64, t1042: f64, t1252: f64, t1261: f64, t12809: f64, t17547: f64, t1797: f64, t20784: f64, t20787: f64, t20789: f64, t20792: f64, t20797: f64, t20802: f64, t20806: f64, t20811: f64, t20817: f64, t3711: f64, t5331: f64, t5340: f64) -> (f64, f64, f64, f64, f64) {
        let (t20820, t20823, t20824, t20825, t20828) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2604(t1246, t20819, t1214, t5819, t5302, t1042, t1252, t1261, t12809, t17547, t1797, t20784, t20787, t20789, t20792, t20797, t20802, t20806, t20811, t20817, t3711, t5331, t5340);
    (t20820, t20823, t20824, t20825, t20828)
}
