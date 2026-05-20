//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta654 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2604;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta654<F: Float>(t1246: F, t20819: F, t1214: F, t5819: F, t5302: F, t1042: F, t1252: F, t1261: F, t12809: F, t17547: F, t1797: F, t20784: F, t20787: F, t20789: F, t20792: F, t20797: F, t20802: F, t20806: F, t20811: F, t20817: F, t3711: F, t5331: F, t5340: F) -> (F, F, F, F, F) {
        let (t20820, t20823, t20824, t20825, t20828) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2604::<F>(t1246, t20819, t1214, t5819, t5302, t1042, t1252, t1261, t12809, t17547, t1797, t20784, t20787, t20789, t20792, t20797, t20802, t20806, t20811, t20817, t3711, t5331, t5340);
    (t20820, t20823, t20824, t20825, t20828)
}
