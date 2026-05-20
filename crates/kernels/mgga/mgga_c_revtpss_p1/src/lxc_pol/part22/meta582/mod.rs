//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta582 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2442;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2443;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2444;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2445;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta582<F: Float>(t18616: F, t827: F, t828: F, t221: F, t2485: F, t6017: F, t2484: F, t125: F, t5962: F, t2747: F, t837: F, t2723: F, t4423: F, t4364: F, t4365: F, t231: F, t4343: F, t10779: F, t14671: F, t6035: F, t10777: F, t14676: F, t18444: F, t14894: F, t14907: F, t14925: F, t14934: F, t18527: F, t18532: F, t2745: F, t4362: F, t825: F, t18330: F, t18343: F, t18361: F, t18405: F, t18454: F, t18489: F, t18524: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t18618, t18622, t18623, t18627, t18629, t18632) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2442::<F>(t18616, t827, t828, t221, t2485, t6017, t2484, t125, t5962, t2747, t837, t2723, t4423);
        let (t18634, t18637) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2443::<F>(t18632, t4364, t4365, t231, t4343);
        let (t18639, t18643, t18647, t18651, t18654) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2444::<F>(t18637, t2747, t4365, t10779, t14671, t6035, t10777, t14676, t18444, t4364, t837, t14894, t14907, t14925, t14934, t18527, t18532, t18618, t18623, t18629, t18634, t2745, t4362, t825);
        let t18657 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2445::<F>(t18330, t18343, t18361, t18405, t18454, t18489, t18524, t18654);
    (t18618, t18622, t18627, t18629, t18632, t18634, t18637, t18639, t18643, t18647, t18651, t18657)
}
