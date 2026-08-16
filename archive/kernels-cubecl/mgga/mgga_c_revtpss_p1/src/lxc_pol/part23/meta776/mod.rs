//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta776 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2580;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta776<F: Float>(t3495: F, t5155: F, t3476: F, t5117: F, t3451: F, t3383: F, t5060: F, t12247: F, t1719: F, t1756: F, t3521: F, t56228: F) -> (F, F, F, F, F, F, F) {
        let (t58307, t58317, t58336, t58339, t58342, t58345, t58404) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2580::<F>(t3495, t5155, t3476, t5117, t3451, t3383, t5060, t12247, t1719, t1756, t3521, t56228);
    (t58307, t58317, t58336, t58339, t58342, t58345, t58404)
}
