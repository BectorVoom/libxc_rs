//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta591 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1963;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1964;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1965;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1966;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta591(t102204: f64, t94771: f64, t122: f64, t72: f64, t8085: f64, t25900: f64, t25899: f64, t28894: f64, t94921: f64, t94802: f64, t28814: f64, t689: f64, t94669: f64, t7528: f64, t96243: f64, t96246: f64, t96249: f64, t96253: f64, t96257: f64, t96260: f64, t96262: f64, t96265: f64, t98050: f64, t2435: f64, t28902: f64, t7515: f64, t98308: f64, t97962: f64, t14110: f64, t96463: f64, t5775: f64, t7492: f64, t2453: f64, t3908: f64, t8086: f64, t28829: f64, t26271: f64, t27884: f64, t28862: f64, t686: f64, t25895: f64, t25924: f64, t4131: f64, t7295: f64, t8094: f64, t8100: f64, t94610: f64, t96269: f64, t96272: f64, t96277: f64, t96280: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t102225, t102234, t102235, t102237, t102239, t102241, t102244) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1963(t102204, t94771, t122, t72, t8085, t25900, t25899, t28894, t94921, t94802, t28814, t689);
        let t102248 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1964(t102244, t94669, t102225, t102237, t102239, t102241, t7528, t96243, t96246, t96249, t96253, t96257, t96260, t96262, t96265, t98050);
        let (t102249, t102253, t102255, t102257, t102261, t102266) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1965(t2435, t28902, t7515, t98308, t97962, t14110, t96463, t5775, t689, t7492, t2453, t3908, t8086);
        let (t102268, t102274, t102282) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1966(t28829, t689, t25899, t26271, t27884, t28862, t686, t72, t25895, t102249, t102253, t102255, t102257, t102261, t102266, t25924, t4131, t7295, t8094, t8100, t94610, t96269, t96272, t96277, t96280);
    (t102234, t102235, t102244, t102248, t102268, t102274, t102282)
}
