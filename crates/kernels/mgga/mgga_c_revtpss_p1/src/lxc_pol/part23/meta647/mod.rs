//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta647 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2371;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta647<F: Float>(t40182: F, t760: F, t36: F, t716: F, t39875: F, t745: F, t9417: F, t2596: F, t39871: F, t2523: F, t9425: F, t10867: F, t860: F) -> (F, F, F, F, F, F, F, F) {
        let (t40184, t40188, t40192, t40194, t40196, t40198, t40205, t40258) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2371::<F>(t40182, t760, t36, t716, t39875, t745, t9417, t2596, t39871, t2523, t9425, t10867, t860);
    (t40184, t40188, t40192, t40194, t40196, t40198, t40205, t40258)
}
