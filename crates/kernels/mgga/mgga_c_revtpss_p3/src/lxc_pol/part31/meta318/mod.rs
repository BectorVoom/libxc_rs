//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta318 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1321;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta318<F: Float>(t3145: F, t334: F, t368: F, t3153: F, t73: F, t246: F, t676: F, t1046: F, t1041: F, t3140: F, t989: F, t3149: F) -> (F, F, F, F, F, F, F) {
        let (t11243, t11249, t11262, t11263, t11264, t11273, t11274) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1321::<F>(t3145, t334, t368, t3153, t73, t246, t676, t1046, t1041, t3140, t989, t3149);
    (t11243, t11249, t11262, t11263, t11264, t11273, t11274)
}
