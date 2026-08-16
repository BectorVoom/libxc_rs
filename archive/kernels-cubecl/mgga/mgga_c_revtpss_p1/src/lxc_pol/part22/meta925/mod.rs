//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta925 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3147;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3148;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta925<F: Float>(t12640: F, t488: F, t17588: F, t3172: F, t3711: F, t1261: F, t17699: F, t17720: F, t3647: F, t12904: F, t5274: F, t12959: F, t17505: F, t17225: F, t11262: F, t5303: F, t5298: F, t127: F, t17352: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t56707, t56713, t56718, t56720, t56726, t56728) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3147::<F>(t12640, t488, t17588, t3172, t3711, t1261, t17699, t17720, t3647, t12904, t5274, t12959, t17505);
        let (t56734, t56739, t56742, t56756) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3148::<F>(t17225, t3647, t11262, t1261, t5303, t3711, t5298, t127, t17352);
    (t56707, t56713, t56718, t56720, t56726, t56728, t56734, t56739, t56742, t56756)
}
