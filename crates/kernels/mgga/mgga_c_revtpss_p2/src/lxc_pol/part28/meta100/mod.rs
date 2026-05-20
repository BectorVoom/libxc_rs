//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta100 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk632;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk633;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk634;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk635;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk636;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk637;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta100<F: Float>(t2311: F, t77: F, t2252: F, t2260: F, t2263: F, t2292: F, t608: F, t628: F, t641: F, t71: F, t85: F, t5: F, t2240: F, t2242: F, t2247: F, t2248: F, t603: F, t644: F, t91: F, t117: F, t116: F, t648: F, t670: F, t94: F, t1310: F, t112: F, t2289: F, t625: F, t666: F, t111: F, t654: F, t665: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2312, t2315) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk632::<F>(t2311, t77, t2252, t2260, t2263, t2292, t608, t628, t641, t71, t85);
        let (t2319, t2320) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk633::<F>(t5, t2240, t2242, t2247, t2248, t2315, t603, t644, t91, t117);
        let t2322 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk634::<F>(t116, t648);
        let t2327 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk635::<F>(t670);
        let (t2328, t2331, t2335, t2336, t2339) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk636::<F>(t2327, t94, t1310, t670, t112, t2289, t625, t666, t111, t654);
        let t2340 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk637::<F>(t665);
    (t2312, t2315, t2319, t2320, t2322, t2327, t2328, t2331, t2335, t2336, t2339, t2340)
}
