//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta843 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2720;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2721;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta843<F: Float>(t21063: F, t3678: F, t17225: F, t5381: F, t1261: F, t20791: F, t3172: F, t13058: F, t20786: F, t11262: F, t3711: F, t6618: F, t21110: F, t17401: F, t17620: F, t17728: F, t489: F, t5219: F, t1256: F, t21335: F, t20900: F, t3153: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t70265, t70270, t70273, t70275, t70278) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2720::<F>(t21063, t3678, t17225, t5381, t1261, t20791, t3172, t13058, t20786, t11262, t3711, t6618);
        let (t70281, t70300, t70303, t70306, t70311) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2721::<F>(t1261, t21110, t3172, t17401, t17620, t17728, t489, t5219, t1256, t21335, t20900, t3153);
    (t70265, t70270, t70273, t70275, t70278, t70281, t70300, t70303, t70306, t70311)
}
