//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta471 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1784;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta471<F: Float>(t1949: F, t231: F, t2645: F, t7076: F, t7014: F, t887: F, t689: F, t7049: F, t786: F, t789: F, t1956: F, t213: F, t25287: F, t25292: F, t25297: F, t25303: F, t25307: F, t25311: F, t25314: F, t25319: F, t25322: F, t25326: F, t25333: F, t25337: F, t25340: F, t25344: F, t257: F, t2772: F, t7053: F, t7067: F, t7070: F, t7083: F) -> (F, F, F, F, F, F, F) {
        let (t25348, t25349, t25352, t25353, t25355, t25356, t25360) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1784::<F>(t1949, t231, t2645, t7076, t7014, t887, t689, t7049, t786, t789, t1956, t213, t25287, t25292, t25297, t25303, t25307, t25311, t25314, t25319, t25322, t25326, t25333, t25337, t25340, t25344, t257, t2772, t7053, t7067, t7070, t7083);
    (t25348, t25349, t25352, t25353, t25355, t25356, t25360)
}
