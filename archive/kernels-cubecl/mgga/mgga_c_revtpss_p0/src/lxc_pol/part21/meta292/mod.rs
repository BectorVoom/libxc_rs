//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta292 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1535;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1536;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1537;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1538;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta292<F: Float>(t45: F, t10439: F, t10440: F, t2401: F, t750: F, t200: F, t2375: F, t606: F, t10326: F, t10356: F, t2258: F, t78: F, zeta_threshold: F, t57: F, t202: F, t2382: F, t81: F, t150: F, t190: F, t80: F, t633: F, t766: F, t83: F, t637: F, t770: F, t10430: F, t10432: F, t10435: F, t10438: F, t198: F, t765: F, t9278: F, t9308: F, t9316: F, t9329: F, t9333: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t10442, t10443, t10444, t10446, t10455) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1535::<F>(t45, t10439, t10440, t2401, t750, t200, t2375, t606, t10326, t10356, t2258, t78, zeta_threshold);
        let (t10457, t10467, t10468, t10469, t10472) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1536::<F>(t57, t202, t2382, t606, t10326, t10356, t2258, t81, t10455, t150, t190, t80, zeta_threshold);
        let t10489 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1537::<F>(t45, t57, t10326, t10356, t10472, t2258, t633, t766, t606, t83, t637, t770, zeta_threshold);
        let t10493 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1538::<F>(t10430, t10432, t10435, t10438, t10442, t10444, t10469, t10489, t198, t765, t9278, t9308, t9316, t9329, t9333);
    (t10442, t10443, t10444, t10446, t10457, t10467, t10468, t10469, t10489, t10493)
}
