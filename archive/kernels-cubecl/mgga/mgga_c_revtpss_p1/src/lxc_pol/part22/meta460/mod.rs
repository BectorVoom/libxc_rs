//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta460 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2135;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2136;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2137;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta460<F: Float>(t15191: F, t15197: F, t11134: F, t11136: F, t11138: F, t11140: F, t11334: F, t11338: F, t11339: F, t11366: F, t11368: F, t15221: F, t15230: F, t11326: F, t15108: F, t15111: F, t15114: F, t15116: F, t15119: F, t15121: F, t15123: F, t15127: F, t15132: F, t15178: F, t15181: F, t15184: F, t15187: F, t15189: F, t15195: F, t15200: F, t15435: F, t15450: F, t935: F, t915: F, t15125: F, t11560: F, t15137: F, t15142: F, t15147: F, t15151: F, t15156: F, t15160: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t15457, t15459, t15472) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2135::<F>(t15191, t15197, t11134, t11136, t11138, t11140, t11334, t11338, t11339, t11366, t11368, t15221, t15230);
        let t15474 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2136::<F>(t11326, t15108, t15111, t15114, t15116, t15119, t15121, t15123, t15127, t15132, t15178, t15181, t15184, t15187, t15189, t15195, t15200, t15435, t15450, t15457, t15459, t15472);
        let (t15475, t15477, t15483, t15484, t15485, t15494) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2137::<F>(t15474, t935, t915, t15127, t15125, t15191, t11134, t11136, t11138, t11140, t11560, t15132, t15137, t15142, t15147, t15151, t15156, t15160, t15189, t15195);
    (t15457, t15459, t15474, t15475, t15477, t15483, t15484, t15485, t15494)
}
