//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta900 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2861;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2862;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2863;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2864;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2865;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2866;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta900<F: Float>(t77047: F, t14330: F, t18575: F, t4186: F, t18259: F, t18306: F, t23210: F, t705: F, t707: F, t1522: F, t61122: F, t40205: F, t50901: F, t40076: F, t40079: F, t40194: F, t40198: F, t77036: F, t77038: F, t77039: F, t77040: F, t77041: F, t77045: F, t10696: F, t14643: F, t14648: F, t14652: F, t1553: F, t18392: F, t18435: F, t18599: F, t18612: F, t227: F, t23114: F, t23148: F, t23235: F, t23238: F, t23241: F, t4343: F, t4415: F, t4416: F, t5962: F, t76421: F, t775: F, t830: F, t832: F, t853: F, t1555: F, t18586: F, t18592: F, t18600: F, t18603: F, t18609: F, t225: F, t229: F, t231: F, t23227: F, t4409: F, t4417: F, t4420: F, t6006: F, t6010: F, t6013: F, t73: F, t76943: F, t76961: F, t76975: F, t76981: F, t77001: F, t77016: F, t77033: F, t833: F, t221: F, t23245: F, t2484: F, t2485: F, t23168: F, t40352: F, t62429: F, t62431: F, t62435: F, t62439: F, t62441: F, t62443: F, t62445: F, t62453: F, t62458: F, t62460: F, t62475: F, t62494: F, t62498: F, t62502: F, t76887: F, t825: F, t827: F, t828: F, t76343: F, t76434: F, t76458: F, t76493: F, t76517: F, t76557: F, t76595: F, t76633: F, t76676: F, t76742: F, t76776: F, t76800: F, t76843: F, t76860: F, t76884: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t77048, t77051, t77053, t77056, t77058, t77059) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2861::<F>(t77047, t14330, t18575, t4186, t18259, t18306, t23210, t705, t707, t1522, t61122, t40205);
        let (t77060, t77061) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2862::<F>(t50901, t40076, t40079, t40194, t40198, t77036, t77038, t77039, t77040, t77041, t77045, t77048, t77051, t77053, t77056, t77058, t77059);
        let t77118 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2863::<F>(t10696, t14643, t14648, t14652, t1553, t18392, t18435, t18599, t18612, t227, t23114, t23148, t23235, t23238, t23241, t4343, t4415, t4416, t5962, t76421, t775, t830, t832, t853);
        let t77120 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2864::<F>(t1555, t18586, t18592, t18600, t18603, t18609, t225, t229, t231, t23227, t4409, t4417, t4420, t6006, t6010, t6013, t73, t76943, t76961, t76975, t76981, t77001, t77016, t77033, t77061, t77118, t833);
        let t77147 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2865::<F>(t221, t23245, t2484, t2485, t23168, t40352, t62429, t62431, t62435, t62439, t62441, t62443, t62445, t62453, t62458, t62460, t62475, t62494, t62498, t62502, t76887, t77120, t825, t827, t828);
        let t77151 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2866::<F>(t76343, t76434, t76458, t76493, t76517, t76557, t76595, t76633, t76676, t76742, t76776, t76800, t76843, t76860, t76884, t77147);
    (t77048, t77051, t77053, t77056, t77058, t77059, t77060, t77120, t77151)
}
