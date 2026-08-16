//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta94 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk530;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk531;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk532;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta94<F: Float>(t107: F, t200: F, t202: F, t205: F, t262: F, t705: F, t716: F, t198: F, t206: F, t890: F, t892: F, t261: F, t125: F, t215: F, t123: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t2357, t2375, t2382, t2393, t2398, t2403) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk530::<F>(t107, t200, t202, t205, t262, t705, t716, t198, t206);
        let (t2404, t2410, t2411, t2434) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk531::<F>(t890, t892, t261, t125, t215);
        let t2435 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk532::<F>(t123, t2434);
    (t2357, t2375, t2382, t2393, t2398, t2403, t2404, t2410, t2411, t2434, t2435)
}
