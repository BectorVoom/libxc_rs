//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta338 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1351;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta338<F: Float>(t11200: F, t378: F, t3043: F, t3042: F, t993: F, t1071: F, t989: F, t3056: F, t988: F, t1031: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t11201, t11210, t11213, t11214, t11220, t11223, t11224, t11238, t11239) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1351::<F>(t11200, t378, t3043, t3042, t993, t1071, t989, t3056, t988, t1031);
    (t11201, t11210, t11213, t11214, t11220, t11223, t11224, t11238, t11239)
}
