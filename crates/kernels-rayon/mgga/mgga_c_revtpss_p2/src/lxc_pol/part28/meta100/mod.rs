//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta100 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk632;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk633;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk634;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk635;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk636;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk637;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta100(t2311: f64, t77: f64, t2252: f64, t2260: f64, t2263: f64, t2292: f64, t608: f64, t628: f64, t641: f64, t71: f64, t85: f64, t5: f64, t2240: f64, t2242: f64, t2247: f64, t2248: f64, t603: f64, t644: f64, t91: f64, t117: f64, t116: f64, t648: f64, t670: f64, t94: f64, t1310: f64, t112: f64, t2289: f64, t625: f64, t666: f64, t111: f64, t654: f64, t665: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2312, t2315) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk632(t2311, t77, t2252, t2260, t2263, t2292, t608, t628, t641, t71, t85);
        let (t2319, t2320) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk633(t5, t2240, t2242, t2247, t2248, t2315, t603, t644, t91, t117);
        let t2322 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk634(t116, t648);
        let t2327 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk635(t670);
        let (t2328, t2331, t2335, t2336, t2339) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk636(t2327, t94, t1310, t670, t112, t2289, t625, t666, t111, t654);
        let t2340 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk637(t665);
    (t2312, t2315, t2319, t2320, t2322, t2327, t2328, t2331, t2335, t2336, t2339, t2340)
}
