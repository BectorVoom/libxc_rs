//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta292 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1163;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta292<F: Float>(t3515: F, t3520: F, t5206: F, t1196: F, t1129: F, t3431: F, t408: F, t1149: F, t3385: F, t3434: F, t421: F, t1187: F, t3495: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t12222, t12224, t12226, t12227, t12228, t12230, t12231, t12233, t12234) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1163::<F>(t3515, t3520, t5206, t1196, t1129, t3431, t408, t1149, t3385, t3434, t421, t1187, t3495);
    (t12222, t12224, t12226, t12227, t12228, t12230, t12231, t12233, t12234)
}
