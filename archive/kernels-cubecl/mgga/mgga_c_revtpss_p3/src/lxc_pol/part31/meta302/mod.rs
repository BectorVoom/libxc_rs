//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta302 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1294;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1295;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta302<F: Float>(t10276: F, t22: F, t2224: F, t588: F, t27: F, t584: F, t20: F, t596: F, t12: F, t583: F, t2231: F, t2237: F, t592: F, t2236: F, t3: F, t25: F, t2246: F, t599: F, t88: F, t89: F, t90: F, t29: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t10278, t10279, t10281, t10284, t10287, t10288, t10290) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1294::<F>(t10276, t22, t2224, t588, t27, t584, t20, t596, t12, t583, t2231, t2237, t592);
        let (t10295, t10301, t10308, t10309) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1295::<F>(t2236, t3, t25, t2246, t599, t88, t89, t90, t29);
    (t10278, t10279, t10281, t10284, t10287, t10288, t10290, t10295, t10301, t10308, t10309)
}
