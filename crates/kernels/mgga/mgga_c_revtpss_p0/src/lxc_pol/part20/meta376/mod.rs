//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta376 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1363;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1364;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1365;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta376<F: Float>(t2237: F, t2482: F, t823: F, t2487: F, t2646: F, t2661: F, t2662: F, t2663: F, t10777: F, t10780: F, t14686: F, t10803: F, t10811: F, t10665: F, t125: F, t10111: F, t849: F, t9720: F, t685: F, t775: F, t855: F, t10489: F, t10770: F, t10771: F, t2477: F, t2745: F, t2747: F, t2749: F, t40251: F, t40393: F, t40395: F, t40399: F, t40403: F, t40409: F, t40411: F, t40413: F, t40421: F, t825: F, t827: F, t828: F, t851: F, t242: F, t240: F, t72: F, t10700: F, t2652: F, t10710: F, t9775: F, t10733: F, t10716: F, t10741: F, t243: F) -> (F, F, F, F, F, F, F, F) {
        let (t40425, t40429, t40438, t40440) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1363::<F>(t2237, t2482, t823, t2487, t2646, t2661, t2662, t2663, t10777, t10780, t14686, t10803, t10811);
        let (t40446, t40457) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1364::<F>(t10665, t125, t10111, t849, t9720, t685, t775, t855, t10489, t10770, t10771, t2477, t2646, t2745, t2747, t2749, t40251, t40393, t40395, t40399, t40403, t40409, t40411, t40413, t40421, t40425, t40429, t40438, t40440, t825, t827, t828, t851);
        let (t40462, t40471, t40473, t40475, t40477, t40479) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1365::<F>(t242, t240, t72, t10700, t2652, t10710, t9775, t10733, t10716, t10741, t10665, t243);
    (t40446, t40457, t40462, t40471, t40473, t40475, t40477, t40479)
}
