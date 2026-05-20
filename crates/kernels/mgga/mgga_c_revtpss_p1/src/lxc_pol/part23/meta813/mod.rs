//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta813 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2658;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta813<F: Float>(t19021: F, t3011: F, t19045: F, t300: F, t379: F, t4746: F, t1679: F, t3057: F, t1078: F, t6244: F, t1678: F, t4743: F) -> (F, F, F, F, F, F) {
        let (t64504, t64510, t64547, t64550, t64555, t64605) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2658::<F>(t19021, t3011, t19045, t300, t379, t4746, t1679, t3057, t1078, t6244, t1678, t4743);
    (t64504, t64510, t64547, t64550, t64555, t64605)
}
