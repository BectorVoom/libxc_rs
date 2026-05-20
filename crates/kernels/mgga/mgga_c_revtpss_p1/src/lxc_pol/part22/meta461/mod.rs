//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta461 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2138;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2139;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2140;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta461<F: Float>(t15494: F, t324: F, t15125: F, t15191: F, t11134: F, t11136: F, t11138: F, t11140: F, t11534: F, t15127: F, t15132: F, t15137: F, t15142: F, t15147: F, t15151: F, t15156: F, t15160: F, t15189: F, t15195: F, t291: F, t11399: F, t15406: F, t15413: F, t15418: F, t15420: F, t15423: F, t15425: F, t15427: F, t15477: F, t1622: F, t2938: F, t2963: F, t2971: F, t2989: F, t4647: F, t4670: F, t15262: F, t15348: F, t15403: F, t300: F, t3007: F, t4724: F, t981: F, t3022: F, t4734: F, t3011: F, t4707: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t15495, t15503, t15504, t15513) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2138::<F>(t15494, t324, t15125, t15191, t11134, t11136, t11138, t11140, t11534, t15127, t15132, t15137, t15142, t15147, t15151, t15156, t15160, t15189, t15195);
        let (t15515, t15516) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2139::<F>(t15513, t291, t11399, t15406, t15413, t15418, t15420, t15423, t15425, t15427, t15477, t15495, t1622, t2938, t2963, t2971, t2989, t4647, t4670);
        let (t15519, t15520, t15522, t15524, t15525) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2140::<F>(t15262, t15348, t15403, t15516, t300, t3007, t4724, t981, t3022, t4734, t3011, t4707);
    (t15495, t15503, t15504, t15513, t15515, t15519, t15520, t15522, t15524, t15525)
}
