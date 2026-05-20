//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta581 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2001;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta581<F: Float>(t3114: F, t93596: F, t11880: F, t7111: F, t11817: F, t7117: F, t3204: F, t7125: F, t11788: F, t1972: F, t3080: F, t7106: F) -> (F, F, F, F, F, F) {
        let (t93670, t93696, t93720, t93728, t93731, t93745) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2001::<F>(t3114, t93596, t11880, t7111, t11817, t7117, t3204, t7125, t11788, t1972, t3080, t7106);
    (t93670, t93696, t93720, t93728, t93731, t93745)
}
