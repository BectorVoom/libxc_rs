//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta473 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1738;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1739;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1740;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1741;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1742;
use chunk5::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1743;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta473(t26563: f64, t789: f64, t231: f64, t7398: f64, t836: f64, t7076: f64, t2061: f64, t2645: f64, t2062: f64, t2453: f64, t2458: f64, t2067: f64, t25383: f64, t25391: f64, t25407: f64, t26529: f64, t26534: f64, t26536: f64, t26538: f64, t26541: f64, t26545: f64, t26547: f64, t26551: f64, t26557: f64, t26558: f64, t26561: f64, t2772: f64, t2829: f64, t7070: f64, t7403: f64, t7420: f64, t887: f64, t26524: f64, t892: f64, t2411: f64, t7427: f64, t11064: f64, t2070: f64, t1940: f64, t2071: f64, t2257: f64, t2403: f64, t25198: f64, t25208: f64, t25211: f64, t25215: f64, t25446: f64, t25449: f64, t25452: f64, t26425: f64, t30: f64, t4541: f64, t605: f64, t7010: f64, t7092: f64, t7428: f64, t7432: f64, t14365: f64, t198: f64, t207: f64, t2394: f64, t2408: f64, t2430: f64, t2832: f64, t775: f64, t890: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t26564, t26568, t26573, t26576, t26578, t26579) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1738(t26563, t789, t231, t7398, t836, t7076, t2061, t2645, t2062, t2453, t2458, t2067, t25383, t25391, t25407, t26529, t26534, t26536, t26538, t26541, t26545, t26547, t26551, t26557, t26558, t26561, t2772, t2829, t7070, t7403, t7420, t887);
        let (t26580, t26581) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1739(t26524, t26579, t892);
        let t26585 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1740(t2411, t7427);
        let t26590 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1741(t11064, t2070);
        let t26601 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1742(t1940, t2071, t2257, t2403, t25198, t25208, t25211, t25215, t25446, t25449, t25452, t26425, t26581, t26585, t26590, t30, t4541, t605, t7010, t7092, t7428, t7432);
        let t26625 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1743(t14365, t1940, t198, t207, t2071, t2394, t2403, t2408, t2430, t26580, t26585, t26590, t2832, t4541, t7428, t7432, t775, t890, t892);
    (t26564, t26568, t26573, t26576, t26578, t26580, t26581, t26585, t26590, t26601, t26625)
}
