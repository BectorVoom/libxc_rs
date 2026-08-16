//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta858 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3005;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3006;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta858<F: Float>(t1565: F, t40781: F, t40488: F, t4354: F, t14862: F, t9775: F, t268: F, t40452: F, t4371: F, t2662: F, t40689: F, t4353: F, t10722: F, t4345: F, t40710: F, t4349: F, t14834: F, t10716: F, t14857: F, t2475: F, t4343: F, t14832: F, t2661: F, t775: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t50370, t50372, t50374, t50377, t50381) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3005::<F>(t1565, t40781, t40488, t4354, t14862, t9775, t268, t40452, t4371, t2662, t40689, t4353);
        let (t50383, t50385, t50387, t50389, t50394) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3006::<F>(t10722, t4345, t40710, t4349, t14834, t9775, t10716, t14857, t2475, t4343, t14832, t2661, t775);
    (t50370, t50372, t50374, t50377, t50381, t50383, t50385, t50387, t50389, t50394)
}
