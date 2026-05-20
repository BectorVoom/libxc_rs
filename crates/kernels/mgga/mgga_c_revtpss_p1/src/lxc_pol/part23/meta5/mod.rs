//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta5 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk40;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk41;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk42;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk43;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk44;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk45;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk46;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta5<F: Float>(t45: F, t78: F, t57: F, t77: F, t71: F, t5: F, t10: F, t11: F, t12: F, t29: F, t9: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t79, t80) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk40::<F>(t45, t78);
        let t81 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk41::<F>(t57);
        let (t82, t83) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk42::<F>(t57, t81);
        let (t84, t85) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk43::<F>(t80, t83, t77);
        let (t88, t89, t90) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk44::<F>(t71, t85);
        let t91 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk45::<F>(t90);
        let t93 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk46::<F>(t5, t10, t11, t12, t29, t9, t91);
    (t79, t80, t81, t82, t83, t84, t85, t88, t89, t90, t91, t93)
}
