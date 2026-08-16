//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta350 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1145;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta350<F: Float>(t187: F, t268: F, t39322: F, t39347: F, t39336: F, t761: F, t39488: F, t2374: F, t39519: F, t39503: F, t39391: F, t39537: F) -> (F, F, F, F, F, F, F, F) {
        let (t40714, t40716, t40721, t40732, t40741, t40743, t40748, t40760) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1145::<F>(t187, t268, t39322, t39347, t39336, t761, t39488, t2374, t39519, t39503, t39391, t39537);
    (t40714, t40716, t40721, t40732, t40741, t40743, t40748, t40760)
}
