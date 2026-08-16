//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta377 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1366;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1367;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1368;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1369;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta377(t231: f64, t2661: f64, t2662: f64, t40479: f64, t10737: f64, t2652: f64, t212: f64, t2237: f64, t225: f64, t816: f64, t2665: f64, t40339: f64, t10627: f64, t10697: f64, t236: f64, t807: f64, t10689: f64, t237: f64, t247: f64, t10709: f64, t10744: f64, t808: f64, t10752: f64, t10905: f64, t2783: f64, t9801: f64, t10745: f64, t10698: f64, t10895: f64, t2394: f64, t2430: f64, t2477: f64, t2730: f64, t40232: f64, t40240: f64, t40462: f64, t40471: f64, t40473: f64, t40475: f64, t40477: f64, t775: f64, t800: f64, t825: f64, t827: f64, t828: f64, t851: f64, t2735: f64, t4503: f64, t10728: f64, t10680: f64, t2710: f64, t2713: f64, t10732: f64, t10674: f64, t2693: f64, t9732: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40482, t40484, t40488, t40489, t40491) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1366(t231, t2661, t2662, t40479, t10737, t2652, t212, t2237, t225, t816, t2665, t40339);
        let (t40503, t40507, t40509, t40511) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1367(t10627, t10697, t236, t807, t10689, t237, t247, t10709, t10744, t808, t10752, t10905);
        let t40520 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1368(t2783, t9801, t10745, t10698, t10895, t2394, t2430, t2477, t2730, t40232, t40240, t40462, t40471, t40473, t40475, t40477, t40482, t40484, t40489, t40491, t40503, t40507, t40509, t40511, t775, t800, t825, t827, t828, t851);
        let (t40523, t40526, t40529, t40532, t40535) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1369(t2735, t4503, t10728, t808, t10680, t2710, t2713, t10732, t10744, t10674, t2693, t9732);
    (t40488, t40491, t40520, t40523, t40526, t40529, t40532, t40535)
}
