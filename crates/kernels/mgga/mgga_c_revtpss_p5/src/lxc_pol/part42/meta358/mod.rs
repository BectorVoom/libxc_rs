//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta358 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1171;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta358<F: Float>(t16708: F, t16710: F, t16712: F, t1256: F, t5258: F, t5262: F, t1804: F, t3655: F, t1786: F, t1260: F, t12987: F, t15687: F, t3623: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t17319, t17320, t17321, t17337, t17339, t17340, t17342, t17344, t17350) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1171::<F>(t16708, t16710, t16712, t1256, t5258, t5262, t1804, t3655, t1786, t1260, t12987, t15687, t3623);
    (t17319, t17320, t17321, t17337, t17339, t17340, t17342, t17344, t17350)
}
