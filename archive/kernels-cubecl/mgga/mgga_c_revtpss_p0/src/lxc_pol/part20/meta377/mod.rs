//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta377 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1366;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1367;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1368;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1369;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta377<F: Float>(t231: F, t2661: F, t2662: F, t40479: F, t10737: F, t2652: F, t212: F, t2237: F, t225: F, t816: F, t2665: F, t40339: F, t10627: F, t10697: F, t236: F, t807: F, t10689: F, t237: F, t247: F, t10709: F, t10744: F, t808: F, t10752: F, t10905: F, t2783: F, t9801: F, t10745: F, t10698: F, t10895: F, t2394: F, t2430: F, t2477: F, t2730: F, t40232: F, t40240: F, t40462: F, t40471: F, t40473: F, t40475: F, t40477: F, t775: F, t800: F, t825: F, t827: F, t828: F, t851: F, t2735: F, t4503: F, t10728: F, t10680: F, t2710: F, t2713: F, t10732: F, t10674: F, t2693: F, t9732: F) -> (F, F, F, F, F, F, F, F) {
        let (t40482, t40484, t40488, t40489, t40491) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1366::<F>(t231, t2661, t2662, t40479, t10737, t2652, t212, t2237, t225, t816, t2665, t40339);
        let (t40503, t40507, t40509, t40511) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1367::<F>(t10627, t10697, t236, t807, t10689, t237, t247, t10709, t10744, t808, t10752, t10905);
        let t40520 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1368::<F>(t2783, t9801, t10745, t10698, t10895, t2394, t2430, t2477, t2730, t40232, t40240, t40462, t40471, t40473, t40475, t40477, t40482, t40484, t40489, t40491, t40503, t40507, t40509, t40511, t775, t800, t825, t827, t828, t851);
        let (t40523, t40526, t40529, t40532, t40535) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1369::<F>(t2735, t4503, t10728, t808, t10680, t2710, t2713, t10732, t10744, t10674, t2693, t9732);
    (t40488, t40491, t40520, t40523, t40526, t40529, t40532, t40535)
}
