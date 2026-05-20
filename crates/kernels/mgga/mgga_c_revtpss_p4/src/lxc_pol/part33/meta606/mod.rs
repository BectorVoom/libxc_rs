//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta606 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2030;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2031;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta606<F: Float>(t26866: F, t3746: F, t12904: F, t7618: F, t3666: F, t7623: F, t12808: F, t29096: F, t3655: F, t7610: F, t12898: F, t2139: F, t12984: F, t7613: F, t12966: F, t2138: F, t12851: F, t2134: F, t3567: F, t8945: F, t26894: F, t29199: F, t3596: F, t37885: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t97232, t97247, t97250, t97261, t97267, t97272) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2030::<F>(t26866, t3746, t12904, t7618, t3666, t7623, t12808, t29096, t3655, t7610, t12898, t2139);
        let (t97288, t97292, t97296, t97304, t97308, t97312) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2031::<F>(t12984, t7613, t12966, t2138, t12851, t2134, t3567, t8945, t26894, t29199, t3596, t37885);
    (t97232, t97247, t97250, t97261, t97267, t97272, t97288, t97292, t97296, t97304, t97308, t97312)
}
