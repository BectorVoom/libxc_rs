//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta341 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1361;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta341<F: Float>(t11643: F, t3127: F, t3135: F, t3172: F, t1041: F, t1024: F, t3105: F, t3151: F, t3153: F, t1052: F, t360: F, t3089: F) -> (F, F, F, F, F, F, F) {
        let (t11644, t11648, t11649, t11656, t11659, t11670, t11671) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1361::<F>(t11643, t3127, t3135, t3172, t1041, t1024, t3105, t3151, t3153, t1052, t360, t3089);
    (t11644, t11648, t11649, t11656, t11659, t11670, t11671)
}
