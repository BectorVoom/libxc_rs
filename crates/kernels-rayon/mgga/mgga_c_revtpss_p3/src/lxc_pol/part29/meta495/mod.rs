//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta495 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1801;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1802;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1803;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta495(t2089: f64, t4292: f64, t670: f64, t8065: f64, t1518: f64, t7474: f64, t1519: f64, t2322: f64, t26399: f64, t28658: f64, t4254: f64, t4257: f64, t651: f64, t7235: f64, t7359: f64, t7374: f64, t7537: f64, t7539: f64, t7732: f64, t7898: f64, t7978: f64, t7988: f64, t8111: f64, t2055: f64, t5517: f64, t72: f64, t8094: f64, t686: f64, t25878: f64, t25895: f64, t1882: f64, t543: f64, t7506: f64, t7301: f64, t27884: f64, t7515: f64, t25921: f64, t26232: f64, t26235: f64, t26238: f64, t26251: f64, t26253: f64, t26263: f64, t26266: f64, t26268: f64, t26272: f64, t7295: f64, t8100: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t28734, t28737, t28750, t28759) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1801(t2089, t4292, t670, t8065, t1518, t7474, t1519, t2322, t26399, t28658, t4254, t4257, t651, t7235, t7359, t7374, t7537, t7539, t7732, t7898, t7978, t7988, t8111);
        let (t28760, t28779, t28780, t28781, t28783, t28791, t28792, t28796) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1802(t2055, t5517, t72, t8094, t686, t25878, t25895, t1882, t543, t7506, t7301, t27884, t7515);
        let t28799 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1803(t25921, t26232, t26235, t26238, t26251, t26253, t26263, t26266, t26268, t26272, t28781, t28783, t28792, t28796, t7295, t8100);
    (t28734, t28737, t28750, t28759, t28760, t28779, t28780, t28791, t28792, t28799)
}
