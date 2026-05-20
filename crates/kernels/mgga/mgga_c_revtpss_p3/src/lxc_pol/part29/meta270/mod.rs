//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta270 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1117;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1118;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1119;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta270<F: Float>(t2102: F, t72: F, t686: F, t7284: F, t7289: F, t1444: F, t2097: F, t7296: F, t1398: F, t543: F, t7301: F, t545: F, t7506: F, t2028: F, t1445: F, t2027: F, t2103: F, t213: F, t561: F, t7292: F, t7295: F, t7495: F, t7498: F, t7507: F, t7511: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t7514, t7515) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1117::<F>(t2102, t72, t686);
        let (t7517, t7519, t7523, t7527, t7528, t7531, t7532) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1118::<F>(t7284, t7515, t7289, t1444, t2097, t7296, t1398, t543, t7301, t545, t7506, t2028);
        let t7535 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1119::<F>(t1445, t2027, t2103, t213, t561, t7292, t7295, t7495, t7498, t7507, t7511, t7517, t7519, t7523, t7528, t7532);
    (t7514, t7515, t7517, t7519, t7523, t7527, t7528, t7531, t7532, t7535)
}
