//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta449 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1635;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1636;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta449(t20795: f64, t5352: f64, t3720: f64, t3153: f64, t6622: f64, t5341: f64, t5333: f64, t1263: f64, t6587: f64, t1122: f64, t1042: f64, t3172: f64, t6624: f64, t1247: f64, t1032: f64, t6564: f64, t1246: f64, t1214: f64, t5819: f64, t5302: f64, t1252: f64, t1261: f64, t12809: f64, t17547: f64, t1797: f64, t20784: f64, t20787: f64, t20789: f64, t20792: f64, t3711: f64, t5331: f64, t5340: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t20797, t20800, t20802, t20806, t20811, t20816) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1635(t20795, t5352, t3720, t3153, t6622, t5341, t5333, t1263, t6587, t1122, t1042, t3172, t6624);
        let (t20819, t20823, t20825, t20828) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1636(t1247, t20816, t1032, t6564, t1246, t1214, t5819, t5302, t1042, t1252, t1261, t12809, t17547, t1797, t20784, t20787, t20789, t20792, t20797, t20802, t20806, t20811, t3711, t5331, t5340);
    (t20797, t20800, t20802, t20806, t20811, t20816, t20819, t20823, t20825, t20828)
}
