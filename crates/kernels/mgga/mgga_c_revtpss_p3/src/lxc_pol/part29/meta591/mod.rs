//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta591 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1963;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1964;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1965;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1966;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta591<F: Float>(t102204: F, t94771: F, t122: F, t72: F, t8085: F, t25900: F, t25899: F, t28894: F, t94921: F, t94802: F, t28814: F, t689: F, t94669: F, t7528: F, t96243: F, t96246: F, t96249: F, t96253: F, t96257: F, t96260: F, t96262: F, t96265: F, t98050: F, t2435: F, t28902: F, t7515: F, t98308: F, t97962: F, t14110: F, t96463: F, t5775: F, t7492: F, t2453: F, t3908: F, t8086: F, t28829: F, t26271: F, t27884: F, t28862: F, t686: F, t25895: F, t25924: F, t4131: F, t7295: F, t8094: F, t8100: F, t94610: F, t96269: F, t96272: F, t96277: F, t96280: F) -> (F, F, F, F, F, F, F) {
        let (t102225, t102234, t102235, t102237, t102239, t102241, t102244) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1963::<F>(t102204, t94771, t122, t72, t8085, t25900, t25899, t28894, t94921, t94802, t28814, t689);
        let t102248 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1964::<F>(t102244, t94669, t102225, t102237, t102239, t102241, t7528, t96243, t96246, t96249, t96253, t96257, t96260, t96262, t96265, t98050);
        let (t102249, t102253, t102255, t102257, t102261, t102266) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1965::<F>(t2435, t28902, t7515, t98308, t97962, t14110, t96463, t5775, t689, t7492, t2453, t3908, t8086);
        let (t102268, t102274, t102282) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1966::<F>(t28829, t689, t25899, t26271, t27884, t28862, t686, t72, t25895, t102249, t102253, t102255, t102257, t102261, t102266, t25924, t4131, t7295, t8094, t8100, t94610, t96269, t96272, t96277, t96280);
    (t102234, t102235, t102244, t102248, t102268, t102274, t102282)
}
