//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta622 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2137;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2138;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2139;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2140;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2141;
use chunk5::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2142;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta622(t14495: f64, t689: f64, t25372: f64, t98799: f64, t25386: f64, t1957: f64, t27357: f64, t14587: f64, t27312: f64, t92838: f64, t25331: f64, t27216: f64, t212: f64, t27265: f64, t780: f64, t1558: f64, t25391: f64, t25392: f64, t25394: f64, t92841: f64, t92844: f64, t92847: f64, t92856: f64, t92858: f64, t92861: f64, t1568: f64, t7063: f64, t25410: f64, t25413: f64, t27299: f64, t93281: f64, t93317: f64, t2439: f64, t7774: f64, t93170: f64, t14489: f64, t1579: f64, t25286: f64, t25292: f64, t25317: f64, t25383: f64, t27199: f64, t27317: f64, t27322: f64, t2771: f64, t2828: f64, t7053: f64, t7070: f64, t7071: f64, t7759: f64, t92870: f64, t92873: f64, t92875: f64, t25304: f64, t27212: f64, t25301: f64, t93371: f64, t27286: f64, t25431: f64, t25411: f64, t27349: f64, t92843: f64, t25314: f64, t25322: f64, t27183: f64, t27267: f64, t4534: f64, t7067: f64, t7766: f64, t7769: f64, t92891: f64, t92895: f64, t92901: f64, t92905: f64, t93118: f64, t27341: f64, t93342: f64, t93364: f64, t27194: f64, t887: f64, t1580: f64, t25334: f64, t2722: f64, t231: f64, t25344: f64, t25416: f64, t27182: f64, t27207: f64, t2723: f64, t4487: f64, t7076: f64, t886: f64, t92922: f64, t92925: f64, t92930: f64, t92935: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t98803, t98806, t98811, t98814, t98815, t98817, t98825) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2137(t14495, t689, t25372, t98799, t25386, t1957, t27357, t14587, t27312, t92838, t25331, t27216);
        let t98831 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2138(t212, t27265, t689, t780, t1558, t25391, t25392, t25394, t92841, t92844, t92847, t92856, t92858, t92861, t98803, t98806, t98811, t98814, t98817, t98825);
        let (t98848, t98857, t98864) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2139(t1568, t7063, t25410, t25413, t27299, t689, t93281, t93317, t2439, t7774, t93170, t14489, t1579, t25286, t25292, t25317, t25383, t27199, t27317, t27322, t2771, t2828, t7053, t7070, t7071, t7759, t92870, t92873, t92875);
        let (t98892, t98895) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2140(t25304, t27212, t25301, t93371, t98857, t27286, t689, t25431, t25411, t27349, t92843, t25314, t25322, t25383, t27183, t27267, t2771, t4534, t7067, t7070, t7766, t7769, t92891, t92895, t92901, t92905, t93118);
        let (t98897, t98907, t98911, t98918, t98920, t98922) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2141(t92838, t98892, t27341, t93342, t93364, t27194, t689, t887, t1580, t2439, t25334, t2722, t7759);
        let t98932 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2142(t1558, t231, t25286, t25317, t25322, t25344, t25383, t25416, t27182, t27199, t27207, t2723, t4487, t7070, t7076, t886, t92922, t92925, t92930, t92935, t98897, t98907, t98911, t98918, t98920, t98922);
    (t98815, t98831, t98848, t98864, t98895, t98922, t98932)
}
