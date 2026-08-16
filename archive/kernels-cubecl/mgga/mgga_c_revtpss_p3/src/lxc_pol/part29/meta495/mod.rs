//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta495 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1801;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1802;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1803;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta495<F: Float>(t2089: F, t4292: F, t670: F, t8065: F, t1518: F, t7474: F, t1519: F, t2322: F, t26399: F, t28658: F, t4254: F, t4257: F, t651: F, t7235: F, t7359: F, t7374: F, t7537: F, t7539: F, t7732: F, t7898: F, t7978: F, t7988: F, t8111: F, t2055: F, t5517: F, t72: F, t8094: F, t686: F, t25878: F, t25895: F, t1882: F, t543: F, t7506: F, t7301: F, t27884: F, t7515: F, t25921: F, t26232: F, t26235: F, t26238: F, t26251: F, t26253: F, t26263: F, t26266: F, t26268: F, t26272: F, t7295: F, t8100: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t28734, t28737, t28750, t28759) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1801::<F>(t2089, t4292, t670, t8065, t1518, t7474, t1519, t2322, t26399, t28658, t4254, t4257, t651, t7235, t7359, t7374, t7537, t7539, t7732, t7898, t7978, t7988, t8111);
        let (t28760, t28779, t28780, t28781, t28783, t28791, t28792, t28796) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1802::<F>(t2055, t5517, t72, t8094, t686, t25878, t25895, t1882, t543, t7506, t7301, t27884, t7515);
        let t28799 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1803::<F>(t25921, t26232, t26235, t26238, t26251, t26253, t26263, t26266, t26268, t26272, t28781, t28783, t28792, t28796, t7295, t8100);
    (t28734, t28737, t28750, t28759, t28760, t28779, t28780, t28791, t28792, t28799)
}
