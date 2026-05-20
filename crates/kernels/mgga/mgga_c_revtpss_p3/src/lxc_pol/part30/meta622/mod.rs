//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta622 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2137;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2138;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2139;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2140;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2141;
use chunk5::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2142;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta622<F: Float>(t14495: F, t689: F, t25372: F, t98799: F, t25386: F, t1957: F, t27357: F, t14587: F, t27312: F, t92838: F, t25331: F, t27216: F, t212: F, t27265: F, t780: F, t1558: F, t25391: F, t25392: F, t25394: F, t92841: F, t92844: F, t92847: F, t92856: F, t92858: F, t92861: F, t1568: F, t7063: F, t25410: F, t25413: F, t27299: F, t93281: F, t93317: F, t2439: F, t7774: F, t93170: F, t14489: F, t1579: F, t25286: F, t25292: F, t25317: F, t25383: F, t27199: F, t27317: F, t27322: F, t2771: F, t2828: F, t7053: F, t7070: F, t7071: F, t7759: F, t92870: F, t92873: F, t92875: F, t25304: F, t27212: F, t25301: F, t93371: F, t27286: F, t25431: F, t25411: F, t27349: F, t92843: F, t25314: F, t25322: F, t27183: F, t27267: F, t4534: F, t7067: F, t7766: F, t7769: F, t92891: F, t92895: F, t92901: F, t92905: F, t93118: F, t27341: F, t93342: F, t93364: F, t27194: F, t887: F, t1580: F, t25334: F, t2722: F, t231: F, t25344: F, t25416: F, t27182: F, t27207: F, t2723: F, t4487: F, t7076: F, t886: F, t92922: F, t92925: F, t92930: F, t92935: F) -> (F, F, F, F, F, F, F) {
        let (t98803, t98806, t98811, t98814, t98815, t98817, t98825) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2137::<F>(t14495, t689, t25372, t98799, t25386, t1957, t27357, t14587, t27312, t92838, t25331, t27216);
        let t98831 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2138::<F>(t212, t27265, t689, t780, t1558, t25391, t25392, t25394, t92841, t92844, t92847, t92856, t92858, t92861, t98803, t98806, t98811, t98814, t98817, t98825);
        let (t98848, t98857, t98864) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2139::<F>(t1568, t7063, t25410, t25413, t27299, t689, t93281, t93317, t2439, t7774, t93170, t14489, t1579, t25286, t25292, t25317, t25383, t27199, t27317, t27322, t2771, t2828, t7053, t7070, t7071, t7759, t92870, t92873, t92875);
        let (t98892, t98895) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2140::<F>(t25304, t27212, t25301, t93371, t98857, t27286, t689, t25431, t25411, t27349, t92843, t25314, t25322, t25383, t27183, t27267, t2771, t4534, t7067, t7070, t7766, t7769, t92891, t92895, t92901, t92905, t93118);
        let (t98897, t98907, t98911, t98918, t98920, t98922) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2141::<F>(t92838, t98892, t27341, t93342, t93364, t27194, t689, t887, t1580, t2439, t25334, t2722, t7759);
        let t98932 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2142::<F>(t1558, t231, t25286, t25317, t25322, t25344, t25383, t25416, t27182, t27199, t27207, t2723, t4487, t7070, t7076, t886, t92922, t92925, t92930, t92935, t98897, t98907, t98911, t98918, t98920, t98922);
    (t98815, t98831, t98848, t98864, t98895, t98922, t98932)
}
