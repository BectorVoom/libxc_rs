//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta458 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1746;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1747;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1748;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta458<F: Float>(t162: F, t47038: F, t47053: F, t189: F, t512: F, t1340: F, t40165: F, t2626: F, t9551: F, t749: F, t9363: F, t268: F, t520: F, t39768: F, t190: F, t22: F, t519: F, t39762: F, t1317: F, t9545: F, t40129: F, t72: F, t757: F, t39807: F, t39813: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t47055, t47057, t47059, t47061, t47064, t47065) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1746::<F>(t162, t47038, t47053, t189, t512, t1340, t40165, t2626, t9551, t749, t9363, t268, t520);
        let (t47067, t47070, t47072, t47074, t47076, t47078) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1747::<F>(t39768, t47065, t190, t22, t519, t39762, t1317, t9545, t1340, t40129, t72, t757, t9363);
        let (t47079, t47080) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1748::<F>(t47078, t39807, t39813, t47057, t47059, t47061, t47064, t47067, t47070, t47072, t47074, t47076);
    (t47055, t47057, t47059, t47061, t47064, t47067, t47070, t47072, t47074, t47076, t47079, t47080)
}
