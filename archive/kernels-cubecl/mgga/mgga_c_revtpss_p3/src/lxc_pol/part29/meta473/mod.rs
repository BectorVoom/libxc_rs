//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta473 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1738;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1739;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1740;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1741;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1742;
use chunk5::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1743;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta473<F: Float>(t26563: F, t789: F, t231: F, t7398: F, t836: F, t7076: F, t2061: F, t2645: F, t2062: F, t2453: F, t2458: F, t2067: F, t25383: F, t25391: F, t25407: F, t26529: F, t26534: F, t26536: F, t26538: F, t26541: F, t26545: F, t26547: F, t26551: F, t26557: F, t26558: F, t26561: F, t2772: F, t2829: F, t7070: F, t7403: F, t7420: F, t887: F, t26524: F, t892: F, t2411: F, t7427: F, t11064: F, t2070: F, t1940: F, t2071: F, t2257: F, t2403: F, t25198: F, t25208: F, t25211: F, t25215: F, t25446: F, t25449: F, t25452: F, t26425: F, t30: F, t4541: F, t605: F, t7010: F, t7092: F, t7428: F, t7432: F, t14365: F, t198: F, t207: F, t2394: F, t2408: F, t2430: F, t2832: F, t775: F, t890: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t26564, t26568, t26573, t26576, t26578, t26579) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1738::<F>(t26563, t789, t231, t7398, t836, t7076, t2061, t2645, t2062, t2453, t2458, t2067, t25383, t25391, t25407, t26529, t26534, t26536, t26538, t26541, t26545, t26547, t26551, t26557, t26558, t26561, t2772, t2829, t7070, t7403, t7420, t887);
        let (t26580, t26581) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1739::<F>(t26524, t26579, t892);
        let t26585 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1740::<F>(t2411, t7427);
        let t26590 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1741::<F>(t11064, t2070);
        let t26601 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1742::<F>(t1940, t2071, t2257, t2403, t25198, t25208, t25211, t25215, t25446, t25449, t25452, t26425, t26581, t26585, t26590, t30, t4541, t605, t7010, t7092, t7428, t7432);
        let t26625 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1743::<F>(t14365, t1940, t198, t207, t2071, t2394, t2403, t2408, t2430, t26580, t26585, t26590, t2832, t4541, t7428, t7432, t775, t890, t892);
    (t26564, t26568, t26573, t26576, t26578, t26580, t26581, t26585, t26590, t26601, t26625)
}
