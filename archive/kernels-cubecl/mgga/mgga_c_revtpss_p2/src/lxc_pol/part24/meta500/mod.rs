//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta500 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1503;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1504;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta500<F: Float>(t1558: F, t231: F, t6016: F, t2782: F, t2797: F, t23167: F, t251: F, t2783: F, t76131: F, t18719: F, t51549: F, t23245: F, t2798: F, t686: F, t72: F, t23359: F, t874: F, t10871: F, t4500: F, t62808: F, t125: F, t23148: F, t23244: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t76163, t76169, t76172, t76182, t76206, t76223) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1503::<F>(t1558, t231, t6016, t2782, t2797, t23167, t251, t2783, t76131, t18719, t51549, t23245, t2798, t686, t72);
        let (t76237, t76242, t76255, t76279, t76284, t76289) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1504::<F>(t23359, t686, t72, t874, t10871, t6016, t4500, t62808, t125, t23148, t23167, t23244);
    (t76163, t76169, t76172, t76182, t76206, t76223, t76237, t76242, t76255, t76279, t76284, t76289)
}
