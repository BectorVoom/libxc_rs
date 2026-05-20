//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta370 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1346;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1347;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta370<F: Float>(t40178: F, t40067: F, t40072: F, t40155: F, t40157: F, t40160: F, t40163: F, t40167: F, t40171: F, t40173: F, t40175: F, t39909: F, t738: F, t745: F, t760: F, t2251: F, t2609: F, t2611: F, t36: F, t716: F, t10440: F, t39875: F, t9417: F, t2596: F, t39871: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t40179, t40180, t40182) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1346::<F>(t40178, t40067, t40072, t40155, t40157, t40160, t40163, t40167, t40171, t40173, t40175, t39909, t738, t745);
        let (t40184, t40187, t40190, t40192, t40194, t40196) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1347::<F>(t40182, t760, t2251, t2609, t2611, t36, t716, t10440, t39875, t745, t9417, t2596, t39871);
    (t40179, t40180, t40182, t40184, t40187, t40190, t40192, t40194, t40196)
}
