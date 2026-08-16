//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta900 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2861;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2862;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2863;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2864;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2865;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2866;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta900(t77047: f64, t14330: f64, t18575: f64, t4186: f64, t18259: f64, t18306: f64, t23210: f64, t705: f64, t707: f64, t1522: f64, t61122: f64, t40205: f64, t50901: f64, t40076: f64, t40079: f64, t40194: f64, t40198: f64, t77036: f64, t77038: f64, t77039: f64, t77040: f64, t77041: f64, t77045: f64, t10696: f64, t14643: f64, t14648: f64, t14652: f64, t1553: f64, t18392: f64, t18435: f64, t18599: f64, t18612: f64, t227: f64, t23114: f64, t23148: f64, t23235: f64, t23238: f64, t23241: f64, t4343: f64, t4415: f64, t4416: f64, t5962: f64, t76421: f64, t775: f64, t830: f64, t832: f64, t853: f64, t1555: f64, t18586: f64, t18592: f64, t18600: f64, t18603: f64, t18609: f64, t225: f64, t229: f64, t231: f64, t23227: f64, t4409: f64, t4417: f64, t4420: f64, t6006: f64, t6010: f64, t6013: f64, t73: f64, t76943: f64, t76961: f64, t76975: f64, t76981: f64, t77001: f64, t77016: f64, t77033: f64, t833: f64, t221: f64, t23245: f64, t2484: f64, t2485: f64, t23168: f64, t40352: f64, t62429: f64, t62431: f64, t62435: f64, t62439: f64, t62441: f64, t62443: f64, t62445: f64, t62453: f64, t62458: f64, t62460: f64, t62475: f64, t62494: f64, t62498: f64, t62502: f64, t76887: f64, t825: f64, t827: f64, t828: f64, t76343: f64, t76434: f64, t76458: f64, t76493: f64, t76517: f64, t76557: f64, t76595: f64, t76633: f64, t76676: f64, t76742: f64, t76776: f64, t76800: f64, t76843: f64, t76860: f64, t76884: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t77048, t77051, t77053, t77056, t77058, t77059) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2861(t77047, t14330, t18575, t4186, t18259, t18306, t23210, t705, t707, t1522, t61122, t40205);
        let (t77060, t77061) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2862(t50901, t40076, t40079, t40194, t40198, t77036, t77038, t77039, t77040, t77041, t77045, t77048, t77051, t77053, t77056, t77058, t77059);
        let t77118 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2863(t10696, t14643, t14648, t14652, t1553, t18392, t18435, t18599, t18612, t227, t23114, t23148, t23235, t23238, t23241, t4343, t4415, t4416, t5962, t76421, t775, t830, t832, t853);
        let t77120 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2864(t1555, t18586, t18592, t18600, t18603, t18609, t225, t229, t231, t23227, t4409, t4417, t4420, t6006, t6010, t6013, t73, t76943, t76961, t76975, t76981, t77001, t77016, t77033, t77061, t77118, t833);
        let t77147 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2865(t221, t23245, t2484, t2485, t23168, t40352, t62429, t62431, t62435, t62439, t62441, t62443, t62445, t62453, t62458, t62460, t62475, t62494, t62498, t62502, t76887, t77120, t825, t827, t828);
        let t77151 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2866(t76343, t76434, t76458, t76493, t76517, t76557, t76595, t76633, t76676, t76742, t76776, t76800, t76843, t76860, t76884, t77147);
    (t77048, t77051, t77053, t77056, t77058, t77059, t77060, t77120, t77151)
}
