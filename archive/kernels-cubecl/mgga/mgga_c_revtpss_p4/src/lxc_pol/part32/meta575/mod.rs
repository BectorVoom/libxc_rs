//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta575 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1901;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta575<F: Float>(t102218: F, t25895: F, t102204: F, t94771: F, t122: F, t72: F, t8085: F, t25900: F, t25899: F, t28894: F, t94921: F, t94802: F) -> (F, F, F, F, F, F, F) {
        let (t102219, t102225, t102234, t102235, t102237, t102239, t102241) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1901::<F>(t102218, t25895, t102204, t94771, t122, t72, t8085, t25900, t25899, t28894, t94921, t94802);
    (t102219, t102225, t102234, t102235, t102237, t102239, t102241)
}
