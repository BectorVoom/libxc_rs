//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta569 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2029;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta569<F: Float>(t25374: F, t93341: F, t25378: F, t11050: F, t25399: F, t11007: F, t1955: F, t7056: F, t93320: F, t25387: F, t93330: F, t25410: F, t93189: F) -> (F, F, F, F, F, F, F, F) {
        let (t93342, t93343, t93346, t93349, t93364, t93365, t93369, t93371) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2029::<F>(t25374, t93341, t25378, t11050, t25399, t11007, t1955, t7056, t93320, t25387, t93330, t25410, t93189);
    (t93342, t93343, t93346, t93349, t93364, t93365, t93369, t93371)
}
