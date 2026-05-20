//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta93 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk530;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk531;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk532;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta93<F: Float>(t2289: F, t45: F, t631: F, t78: F, t57: F, t635: F, t81: F, t116: F, t648: F, t112: F, t625: F, t666: F, t111: F, t654: F, t99: F, t107: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t2290, t2297, t2299, t2304, t2306, t2322) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk530::<F>(t2289, t45, t631, t78, t57, t635, t81, t116, t648);
        let (t2335, t2336, t2339) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk531::<F>(t112, t2289, t625, t666, t111, t654);
        let (t2349, t2357) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk532::<F>(t99, t107);
    (t2290, t2297, t2299, t2304, t2306, t2322, t2335, t2336, t2339, t2349, t2357)
}
