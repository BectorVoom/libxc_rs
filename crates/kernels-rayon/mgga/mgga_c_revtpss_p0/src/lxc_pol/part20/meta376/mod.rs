//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta376 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1363;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1364;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1365;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta376(t2237: f64, t2482: f64, t823: f64, t2487: f64, t2646: f64, t2661: f64, t2662: f64, t2663: f64, t10777: f64, t10780: f64, t14686: f64, t10803: f64, t10811: f64, t10665: f64, t125: f64, t10111: f64, t849: f64, t9720: f64, t685: f64, t775: f64, t855: f64, t10489: f64, t10770: f64, t10771: f64, t2477: f64, t2745: f64, t2747: f64, t2749: f64, t40251: f64, t40393: f64, t40395: f64, t40399: f64, t40403: f64, t40409: f64, t40411: f64, t40413: f64, t40421: f64, t825: f64, t827: f64, t828: f64, t851: f64, t242: f64, t240: f64, t72: f64, t10700: f64, t2652: f64, t10710: f64, t9775: f64, t10733: f64, t10716: f64, t10741: f64, t243: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40425, t40429, t40438, t40440) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1363(t2237, t2482, t823, t2487, t2646, t2661, t2662, t2663, t10777, t10780, t14686, t10803, t10811);
        let (t40446, t40457) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1364(t10665, t125, t10111, t849, t9720, t685, t775, t855, t10489, t10770, t10771, t2477, t2646, t2745, t2747, t2749, t40251, t40393, t40395, t40399, t40403, t40409, t40411, t40413, t40421, t40425, t40429, t40438, t40440, t825, t827, t828, t851);
        let (t40462, t40471, t40473, t40475, t40477, t40479) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1365(t242, t240, t72, t10700, t2652, t10710, t9775, t10733, t10716, t10741, t10665, t243);
    (t40446, t40457, t40462, t40471, t40473, t40475, t40477, t40479)
}
