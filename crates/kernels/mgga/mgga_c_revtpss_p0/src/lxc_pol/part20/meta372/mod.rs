//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta372 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1352;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1353;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta372<F: Float>(t10867: F, t860: F, t2722: F, t2723: F, t10069: F, t10929: F, t138: F, t785: F, t9302: F, t2786: F, t10073: F, t10920: F, t231: F, t2760: F, t2782: F, t2783: F, t836: F, t10871: F, t14545: F, t39709: F, t2645: F, t234: F, t39545: F, t685: F, t875: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t40258, t40262, t40263, t40267, t40270, t40271, t40273) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1352::<F>(t10867, t860, t2722, t2723, t10069, t10929, t138, t785, t9302, t2786, t10073, t10920);
        let (t40278, t40282, t40284, t40294) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1353::<F>(t231, t2760, t2782, t2783, t836, t10871, t14545, t39709, t2645, t234, t39545, t685, t875);
    (t40258, t40262, t40263, t40267, t40270, t40271, t40273, t40278, t40282, t40284, t40294)
}
