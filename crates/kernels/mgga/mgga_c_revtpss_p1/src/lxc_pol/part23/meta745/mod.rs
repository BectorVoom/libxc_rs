//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta745 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2528;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2529;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta745<F: Float>(t10111: F, t22: F, t4518: F, t231: F, t39698: F, t4494: F, t10073: F, t14509: F, t10069: F, t40921: F, t4496: F, t14537: F, t10504: F, t136: F, t2457: F, t4533: F, t14473: F, t9303: F, t14477: F, t2435: F, t14482: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t51660, t51676, t51683, t51685, t51686, t51688) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2528::<F>(t10111, t22, t4518, t231, t39698, t4494, t10073, t14509, t10069, t40921, t4496, t14537);
        let (t51704, t51727, t51733, t51742, t51756) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2529::<F>(t10069, t14537, t10504, t136, t2457, t4533, t14473, t9303, t14477, t2435, t10073, t14482);
    (t51660, t51676, t51683, t51685, t51686, t51688, t51704, t51727, t51733, t51742, t51756)
}
