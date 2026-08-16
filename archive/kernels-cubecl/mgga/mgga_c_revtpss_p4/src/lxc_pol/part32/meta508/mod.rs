//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta508 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1798;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta508<F: Float>(t225: F, t30247: F, t26304: F, t30105: F, t1882: F, t543: F, t8085: F, t7301: F, t2097: F, t6843: F, t30225: F, t6895: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t30248, t30252, t30256, t30257, t30261, t30262, t30266, t30267, t30278) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1798::<F>(t225, t30247, t26304, t30105, t1882, t543, t8085, t7301, t2097, t6843, t30225, t6895);
    (t30248, t30252, t30256, t30257, t30261, t30262, t30266, t30267, t30278)
}
