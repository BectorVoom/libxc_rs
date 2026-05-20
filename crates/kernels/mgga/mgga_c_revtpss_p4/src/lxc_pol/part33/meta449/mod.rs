//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta449 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1635;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1636;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta449<F: Float>(t20795: F, t5352: F, t3720: F, t3153: F, t6622: F, t5341: F, t5333: F, t1263: F, t6587: F, t1122: F, t1042: F, t3172: F, t6624: F, t1247: F, t1032: F, t6564: F, t1246: F, t1214: F, t5819: F, t5302: F, t1252: F, t1261: F, t12809: F, t17547: F, t1797: F, t20784: F, t20787: F, t20789: F, t20792: F, t3711: F, t5331: F, t5340: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t20797, t20800, t20802, t20806, t20811, t20816) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1635::<F>(t20795, t5352, t3720, t3153, t6622, t5341, t5333, t1263, t6587, t1122, t1042, t3172, t6624);
        let (t20819, t20823, t20825, t20828) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1636::<F>(t1247, t20816, t1032, t6564, t1246, t1214, t5819, t5302, t1042, t1252, t1261, t12809, t17547, t1797, t20784, t20787, t20789, t20792, t20797, t20802, t20806, t20811, t3711, t5331, t5340);
    (t20797, t20800, t20802, t20806, t20811, t20816, t20819, t20823, t20825, t20828)
}
