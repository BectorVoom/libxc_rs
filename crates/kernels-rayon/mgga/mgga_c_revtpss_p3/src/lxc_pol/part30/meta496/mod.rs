//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta496 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1848;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1849;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1850;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta496(t2247: f64, t26754: f64, t2282: f64, t55: f64, t2251: f64, t2258: f64, t25137: f64, t7571: f64, t72: f64, t1927: f64, t6977: f64, t7575: f64, t2122: f64, t25146: f64, t10309: f64, t7565: f64, t25163: f64, t1923: f64, t2123: f64, t25102: f64, t25110: f64, t25114: f64, t25117: f64, t25120: f64, t25150: f64, t25159: f64, t25162: f64, t26749: f64, t6954: f64, t6960: f64, t6963: f64, t7566: f64, t7576: f64, t7579: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t26755, t26776, t26781, t26782, t26783, t26786) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1848(t2247, t26754, t2282, t55, t2251, t2258, t25137, t7571, t72, t1927, t6977, t7575);
        let (t26789, t26792) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1849(t2122, t25146, t10309, t7565);
        let (t26795, t26798) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1850(t2122, t25163, t1923, t2123, t25102, t25110, t25114, t25117, t25120, t25150, t25159, t25162, t26749, t26755, t26783, t26786, t26789, t26792, t6954, t6960, t6963, t7566, t7576, t7579);
    (t26755, t26776, t26781, t26782, t26783, t26786, t26789, t26792, t26795, t26798)
}
