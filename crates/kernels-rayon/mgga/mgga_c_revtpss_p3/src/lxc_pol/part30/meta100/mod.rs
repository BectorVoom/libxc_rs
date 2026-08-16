//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta100 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk629;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk630;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk631;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk632;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk633;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta100(t2291: f64, t38: f64, t45: f64, t631: f64, t78: f64, t57: f64, t635: f64, t81: f64, t2251: f64, t2258: f64, t633: f64, t637: f64, t77: f64, t2252: f64, t2260: f64, t2263: f64, t608: f64, t628: f64, t641: f64, t71: f64, t85: f64, t5: f64, t2240: f64, t2242: f64, t2247: f64, t2248: f64, t603: f64, t644: f64, t91: f64, t117: f64, t116: f64, t648: f64, t670: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2292, t2297, t2299, t2304, t2306, t2311) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk629(t2291, t38, t45, t631, t78, t57, t635, t81, t2251, t2258, t633, t637);
        let (t2312, t2315) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk630(t2311, t77, t2252, t2260, t2263, t2292, t608, t628, t641, t71, t85);
        let (t2319, t2320) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk631(t5, t2240, t2242, t2247, t2248, t2315, t603, t644, t91, t117);
        let t2322 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk632(t116, t648);
        let t2327 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk633(t670);
    (t2292, t2297, t2299, t2304, t2306, t2311, t2312, t2315, t2319, t2320, t2322, t2327)
}
