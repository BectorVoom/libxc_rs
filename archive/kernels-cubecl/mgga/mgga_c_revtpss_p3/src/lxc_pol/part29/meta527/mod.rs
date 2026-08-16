//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta527 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1855;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta527<F: Float>(t665: F, t94975: F, t2339: F, t624: F, t2340: F, t2366: F, t25823: F, t10208: F, t68: F, t25081: F, t7234: F, t1464: F, t7541: F) -> (F, F, F, F, F, F, F) {
        let (t94976, t94978, t94979, t94981, t94982, t95088, t95182) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1855::<F>(t665, t94975, t2339, t624, t2340, t2366, t25823, t10208, t68, t25081, t7234, t1464, t7541);
    (t94976, t94978, t94979, t94981, t94982, t95088, t95182)
}
