//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta844 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2722;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2723;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta844<F: Float>(t1261: F, t20981: F, t3172: F, t13033: F, t21188: F, t20985: F, t20820: F, t3704: F, t17720: F, t5381: F, t20810: F, t3711: F, t17412: F, t5378: F, t17416: F, t12915: F, t20721: F, t247: F, t5384: F, t21192: F, t3647: F, t21143: F, t3636: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t70369, t70373, t70376, t70378, t70382, t70394) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2722::<F>(t1261, t20981, t3172, t13033, t21188, t20985, t20820, t3704, t17720, t5381, t20810, t3711);
        let (t70403, t70405, t70411, t70427, t70432) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2723::<F>(t17412, t5378, t17416, t5381, t12915, t20721, t247, t5384, t21192, t3647, t21143, t3636);
    (t70369, t70373, t70376, t70378, t70382, t70394, t70403, t70405, t70411, t70427, t70432)
}
