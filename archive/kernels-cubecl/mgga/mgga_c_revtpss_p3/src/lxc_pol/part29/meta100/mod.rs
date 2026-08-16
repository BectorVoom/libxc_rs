//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta100 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk603;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk604;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk605;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk606;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk607;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta100<F: Float>(t2291: F, t38: F, t45: F, t631: F, t78: F, t57: F, t635: F, t81: F, t2251: F, t2258: F, t633: F, t637: F, t77: F, t2252: F, t2260: F, t2263: F, t608: F, t628: F, t641: F, t71: F, t85: F, t5: F, t2240: F, t2242: F, t2247: F, t2248: F, t603: F, t644: F, t91: F, t117: F, t116: F, t648: F, t670: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2292, t2297, t2299, t2304, t2306, t2311) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk603::<F>(t2291, t38, t45, t631, t78, t57, t635, t81, t2251, t2258, t633, t637);
        let (t2312, t2315) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk604::<F>(t2311, t77, t2252, t2260, t2263, t2292, t608, t628, t641, t71, t85);
        let (t2319, t2320) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk605::<F>(t5, t2240, t2242, t2247, t2248, t2315, t603, t644, t91, t117);
        let t2322 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk606::<F>(t116, t648);
        let t2327 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk607::<F>(t670);
    (t2292, t2297, t2299, t2304, t2306, t2311, t2312, t2315, t2319, t2320, t2322, t2327)
}
