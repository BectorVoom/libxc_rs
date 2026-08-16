//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta94 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk668;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk669;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk670;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta94<F: Float>(t670: F, t94: F, t1310: F, t112: F, t2289: F, t625: F, t666: F, t111: F, t654: F, t665: F, t613: F, t99: F, tau0: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t2327 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk668::<F>(t670);
        let (t2328, t2331, t2335, t2336, t2339) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk669::<F>(t2327, t94, t1310, t670, t112, t2289, t625, t666, t111, t654);
        let (t2340, t2341, t2344, t2349) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk670::<F>(t665, t2339, t613, t99, tau0);
    (t2327, t2328, t2331, t2335, t2336, t2339, t2340, t2341, t2344, t2349)
}
