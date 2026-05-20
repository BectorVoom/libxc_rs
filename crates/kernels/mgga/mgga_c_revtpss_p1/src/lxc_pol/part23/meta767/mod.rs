//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta767 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2567;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta767<F: Float>(t12904: F, t5274: F, t11262: F, t1261: F, t5303: F, t3711: F, t5298: F, t127: F, t17352: F, t5293: F, t5269: F, t3140: F, t5216: F) -> (F, F, F, F, F, F, F) {
        let (t56727, t56740, t56742, t56756, t56786, t56791, t56802) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2567::<F>(t12904, t5274, t11262, t1261, t5303, t3711, t5298, t127, t17352, t5293, t5269, t3140, t5216);
    (t56727, t56740, t56742, t56756, t56786, t56791, t56802)
}
