//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta403 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1339;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1340;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta403<F: Float>(t10868: F, t820: F, t843: F, t2482: F, t27: F, t823: F, t9948: F, t2681: F, t2719: F, t10111: F, t9720: F, t2237: F, t849: F, t242: F, t240: F, t72: F, t212: F, t225: F, t816: F, t10689: F, t237: F, t247: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t40348, t40352, t40360, t40398, t40406, t40424) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1339::<F>(t10868, t820, t843, t2482, t27, t823, t9948, t2681, t2719, t10111, t9720, t2237);
        let (t40452, t40462, t40488, t40507) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1340::<F>(t10111, t849, t9720, t242, t240, t72, t212, t2237, t225, t816, t10689, t237, t247);
    (t40348, t40352, t40360, t40398, t40406, t40424, t40452, t40462, t40488, t40507)
}
