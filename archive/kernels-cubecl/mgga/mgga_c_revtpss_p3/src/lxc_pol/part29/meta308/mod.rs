//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta308 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1206;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1207;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta308<F: Float>(t15: F, t588: F, t11: F, t2: F, t22: F, t2224: F, t27: F, t584: F, t20: F, t596: F, t12: F, t583: F, t2231: F, t2237: F, t592: F, t2236: F, t3: F, t25: F, t2240: F, t602: F, t2246: F, t599: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t10275, t10278, t10279, t10281, t10284, t10285) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1206::<F>(t15, t588, t11, t2, t22, t2224, t27, t584, t20, t596, t12, t583);
        let (t10287, t10288, t10290, t10295, t10298, t10301) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1207::<F>(t10285, t27, t2231, t596, t2237, t592, t2236, t3, t25, t2240, t602, t2246, t599);
    (t10275, t10278, t10279, t10281, t10284, t10287, t10288, t10290, t10295, t10298, t10301)
}
