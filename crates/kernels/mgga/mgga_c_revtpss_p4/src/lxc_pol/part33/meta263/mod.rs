//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta263 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1177;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1178;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta263<F: Float>(t1444: F, t2022: F, t7296: F, t1385: F, t1426: F, t1398: F, t543: F, t545: F, t7274: F, t2028: F, t1445: F, t2027: F, t2030: F, t213: F, t561: F, t7245: F, t7248: F, t7275: F, t7279: F, t7288: F, t7291: F, t7292: F, t7295: F) -> (F, F, F, F, F, F, F) {
        let (t7298, t7301) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1177::<F>(t1444, t2022, t7296, t1385, t1426);
        let (t7303, t7304, t7307, t7308, t7311) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1178::<F>(t1398, t2022, t543, t7301, t545, t7274, t2028, t1445, t2027, t2030, t213, t561, t7245, t7248, t7275, t7279, t7288, t7291, t7292, t7295, t7298);
    (t7298, t7301, t7303, t7304, t7307, t7308, t7311)
}
