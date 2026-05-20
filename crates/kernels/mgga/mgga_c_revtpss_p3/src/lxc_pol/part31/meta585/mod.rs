//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta585 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2007;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta585<F: Float>(t25610: F, t27668: F, t25460: F, t3057: F, t25698: F, t378: F, t8521: F, t11108: F, t7177: F, t1989: F, t41937: F, t1113: F, t2411: F) -> (F, F, F, F, F, F) {
        let (t94085, t94095, t94122, t94142, t94149, t94245) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2007::<F>(t25610, t27668, t25460, t3057, t25698, t378, t8521, t11108, t7177, t1989, t41937, t1113, t2411);
    (t94085, t94095, t94122, t94142, t94149, t94245)
}
