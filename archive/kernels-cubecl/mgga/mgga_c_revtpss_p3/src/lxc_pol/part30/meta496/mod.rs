//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta496 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1848;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1849;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1850;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta496<F: Float>(t2247: F, t26754: F, t2282: F, t55: F, t2251: F, t2258: F, t25137: F, t7571: F, t72: F, t1927: F, t6977: F, t7575: F, t2122: F, t25146: F, t10309: F, t7565: F, t25163: F, t1923: F, t2123: F, t25102: F, t25110: F, t25114: F, t25117: F, t25120: F, t25150: F, t25159: F, t25162: F, t26749: F, t6954: F, t6960: F, t6963: F, t7566: F, t7576: F, t7579: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t26755, t26776, t26781, t26782, t26783, t26786) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1848::<F>(t2247, t26754, t2282, t55, t2251, t2258, t25137, t7571, t72, t1927, t6977, t7575);
        let (t26789, t26792) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1849::<F>(t2122, t25146, t10309, t7565);
        let (t26795, t26798) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1850::<F>(t2122, t25163, t1923, t2123, t25102, t25110, t25114, t25117, t25120, t25150, t25159, t25162, t26749, t26755, t26783, t26786, t26789, t26792, t6954, t6960, t6963, t7566, t7576, t7579);
    (t26755, t26776, t26781, t26782, t26783, t26786, t26789, t26792, t26795, t26798)
}
